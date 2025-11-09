use crate::database::entity::user;
use crate::state::ServerState;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use log::{error, info};
use std::sync::Arc;
use uuid::Uuid;
use wabble_core::crypto::secret::Secret;
use wabble_core::crypto::verify_secret;
use wabble_core::message::client::{ClientAdminCommand, ClientMessage};
use wabble_core::message::server::{ServerAdminMessage, ServerError, ServerMessage, ServerResult};
use wabble_core::types::friend_info::FriendInfo;
use wabble_core::types::friend_request_info::FriendRequestInfo;
use wabble_core::types::friendship_status::FriendshipStatus;
use wabble_core::types::user_permissions::UserPermissions;

pub struct WebsocketConnection {
    id: Uuid,
    state: Arc<ServerState>,
}

impl WebsocketConnection {
    pub fn new(conn_id: Uuid, state: Arc<ServerState>) -> Self {
        Self { id: conn_id, state }
    }

    pub async fn handle_receive(&self, mut ws_receive: SplitStream<WebSocket>) {
        while let Some(Ok(message)) = ws_receive.next().await {
            match message {
                Message::Binary(data) => {
                    match bincode::decode_from_slice(data.as_ref(), bincode::config::standard()) {
                        Ok((client_message, _)) => {
                            self.handle_client_message(client_message).await;
                        }
                        Err(e) => {
                            error!("[{}] Failed to decode message: {e}", self.id);
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            };
        }
    }

    async fn send_to_connection(&self, message: ServerMessage) {
        self.state
            .connections
            .send_to_connection(self.id, message)
            .await;
    }

    async fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        self.state.connections.send_to_user(user_id, message).await;
    }

    async fn verify_logged_in(&self) -> ServerResult<user::Model> {
        let Some(user_id) = self.state.connections.get_connection_user(self.id) else {
            return Err(ServerError::Unauthorized);
        };

        let Some(user) = self.state.stores.user.find_by_id(user_id).await? else {
            return Err(ServerError::Unauthorized);
        };

        Ok(user)
    }

    async fn verify_permissions(&self, permissions: UserPermissions) -> ServerResult<()> {
        let user = self.verify_logged_in().await?;
        if !user.has_permissions(permissions) {
            return Err(ServerError::Forbidden);
        };

        Ok(())
    }

    async fn handle_client_message(&self, message: ClientMessage) {
        let result = match message {
            ClientMessage::Ping => {
                self.send_to_connection(ServerMessage::Pong).await;
                Ok(())
            }
            ClientMessage::Login { username, password } => {
                self.handle_login(username, password).await
            }
            ClientMessage::LoginSession { id, token } => self.handle_login_session(id, token).await,
            ClientMessage::Register {
                username,
                password,
                invite_code,
            } => self.handle_register(username, password, invite_code).await,
            ClientMessage::RequestSessionToken => self.handle_request_session_token().await,
            ClientMessage::SendFriendRequest { friend_code } => {
                self.handle_send_friend_request(friend_code).await
            }
            ClientMessage::AcceptFriendRequest { user_id } => {
                self.handle_accept_friend_request(user_id).await
            }
            ClientMessage::BlockFriendRequest { user_id } => {
                self.handle_block_friend_request(user_id).await
            }
            ClientMessage::RetrieveFriendRequests => self.retrieve_friend_requests().await,
            ClientMessage::RetrieveFriends => self.retrieve_friends().await,
            ClientMessage::Admin(admin_command) => self.handle_admin_command(admin_command).await,
        };

        if let Err(err) = result {
            self.send_to_connection(ServerMessage::Error(err)).await;
        }
    }

    async fn handle_login(&self, username: String, password: Secret) -> ServerResult<()> {
        let user = self
            .state
            .stores
            .user
            .find_by_username(&username)
            .await?
            .ok_or(ServerError::InvalidCredentials)?;

        if !verify_secret(&password, &user.password_hash) {
            return Err(ServerError::InvalidCredentials);
        };
        drop(password);

        if self.state.connections.has_connection_user(self.id, user.id) {
            self.send_to_connection(ServerMessage::Authenticated(user.get_me()))
                .await;
            return Ok(());
        };

        self.state.connections.register_user(self.id, user.id);
        self.send_to_connection(ServerMessage::Authenticated(user.get_me()))
            .await;

        info!(
            "[{}] Logged in as '{}' via regular login",
            self.id, username
        );

        Ok(())
    }

    async fn handle_login_session(&self, id: String, token: Secret) -> ServerResult<()> {
        let user_id = Uuid::parse_str(&id).map_err(|_| ServerError::SessionInvalid)?;
        let session = self
            .state
            .stores
            .user_session
            .find(user_id)
            .await?
            .ok_or(ServerError::SessionInvalid)?;

        if !verify_secret(&token, &session.token_hash) {
            return Err(ServerError::SessionInvalid);
        };
        drop(token);

        let user = self
            .state
            .stores
            .user
            .find_by_id(user_id)
            .await?
            .ok_or(ServerError::SessionInvalid)?;
        self.state.connections.register_user(self.id, user.id);
        self.send_to_connection(ServerMessage::Authenticated(user.get_me()))
            .await;

        info!(
            "[{}] Logged in as '{}' via session token",
            self.id, user.name
        );

        Ok(())
    }

    async fn handle_register(
        &self,
        username: String,
        password: Secret,
        invite_code: String,
    ) -> ServerResult<()> {
        let user = self
            .state
            .services
            .user
            .register_user(username, password, invite_code)
            .await?;
        self.state.connections.register_user(self.id, user.id);
        self.send_to_connection(ServerMessage::Authenticated(user.get_me()))
            .await;

        info!("[{}] Registered as '{}'", self.id, user.name);

        Ok(())
    }

    async fn handle_request_session_token(&self) -> ServerResult<()> {
        let user = self.verify_logged_in().await?;
        let token = self.state.services.user.get_session_token(&user).await?;
        self.send_to_connection(ServerMessage::SessionToken {
            id: user.id.to_string(),
            token,
        })
        .await;
        Ok(())
    }

    async fn handle_send_friend_request(&self, friend_code: String) -> ServerResult<()> {
        let user = self.verify_logged_in().await?;
        let friendship = self
            .state
            .services
            .friendship
            .send_request(&user, friend_code)
            .await?;

        self.send_to_connection(ServerMessage::FriendRequestSent)
            .await;

        let friend_id = friendship.get_other_user_id(&user.id);
        if self.state.connections.is_online(friend_id) {
            let info = FriendRequestInfo {
                user_id: user.id.to_string(),
                user_name: user.name,
            };
            self.send_to_user(friend_id, ServerMessage::FriendRequestReceived(info))
                .await;
        }

        Ok(())
    }

    async fn handle_accept_friend_request(&self, user_id: String) -> ServerResult<()> {
        let user = self.verify_logged_in().await?;
        let friendship = self
            .state
            .services
            .friendship
            .accept_request(&user, user_id)
            .await?;

        self.send_to_connection(ServerMessage::FriendRequestAccepted)
            .await;

        let friend_id = friendship.get_other_user_id(&user.id);
        if self.state.connections.is_online(friend_id) {
            let info = FriendInfo {
                user_id: user.id.to_string(),
                user_name: user.name,
                timestamp_utc: friendship.created_at.and_utc().timestamp(),
                is_online: self.state.connections.is_online(user.id),
            };
            self.send_to_user(friend_id, ServerMessage::FriendRequestWasAccepted(info))
                .await;
        }

        Ok(())
    }

    async fn handle_block_friend_request(&self, user_id: String) -> ServerResult<()> {
        let user = self.verify_logged_in().await?;
        self.state
            .services
            .friendship
            .block_request(&user, user_id)
            .await?;
        self.send_to_connection(ServerMessage::FriendRequestBlocked)
            .await;
        Ok(())
    }

    async fn retrieve_friend_requests(&self) -> ServerResult<()> {
        let user = self.verify_logged_in().await?;
        let infos = self
            .state
            .services
            .friendship
            .get_friend_requests(&user)
            .await?;
        self.send_to_connection(ServerMessage::FriendRequests(infos))
            .await;
        Ok(())
    }

    async fn retrieve_friends(&self) -> ServerResult<()> {
        let user = self.verify_logged_in().await?;
        let infos = self
            .state
            .services
            .friendship
            .get_friends(&user)
            .await?
            .into_iter()
            .map(|friend_info| {
                let is_online = Uuid::parse_str(&friend_info.user_id)
                    .map(|uuid| self.state.connections.is_online(uuid))
                    .unwrap_or(false);

                FriendInfo {
                    is_online,
                    ..friend_info
                }
            })
            .collect();
        self.send_to_connection(ServerMessage::Friends(infos)).await;
        Ok(())
    }

    async fn handle_admin_command(&self, admin_command: ClientAdminCommand) -> ServerResult<()> {
        match admin_command {
            ClientAdminCommand::GenerateInviteCodes(amount) => {
                self.handle_admin_generate_invites(amount).await
            }
            ClientAdminCommand::RetrieveInviteCodes => self.handle_admin_retrieve_invites().await,
        }
    }

    async fn handle_admin_generate_invites(&self, amount: u8) -> ServerResult<()> {
        self.verify_permissions(UserPermissions::INVITE_MANAGER)
            .await?;

        self.state
            .stores
            .invite_code
            .create_many(amount)
            .await
            .map_err(|_| ServerError::Database)?;

        Ok(())
    }

    async fn handle_admin_retrieve_invites(&self) -> ServerResult<()> {
        self.verify_permissions(UserPermissions::INVITE_MANAGER)
            .await?;

        let invite_codes = self
            .state
            .stores
            .invite_code
            .find_all()
            .await
            .map_err(|_| ServerError::Database)?
            .iter()
            .map(|invite| invite.code.to_string())
            .collect();

        self.send_to_connection(ServerMessage::Admin(ServerAdminMessage::InviteCodes(
            invite_codes,
        )))
        .await;

        Ok(())
    }
}
