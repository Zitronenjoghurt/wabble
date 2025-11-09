use crate::database::entity::{user, user_friendship};
use crate::stores::Stores;
use std::sync::Arc;
use uuid::Uuid;
use wabble_core::message::server::{ServerError, ServerResult};
use wabble_core::types::friend_info::FriendInfo;
use wabble_core::types::friend_request_info::FriendRequestInfo;
use wabble_core::types::friendship_status::FriendshipStatus;

pub struct FriendshipService {
    stores: Arc<Stores>,
}

impl FriendshipService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub async fn send_request(
        &self,
        user: &user::Model,
        friend_code: String,
    ) -> ServerResult<user_friendship::Model> {
        let friend = self
            .stores
            .user
            .find_by_friend_code(&friend_code)
            .await?
            .ok_or(ServerError::FriendCodeInvalid)?;

        if friend.id == user.id {
            return Err(ServerError::FriendCodeInvalid);
        }

        let id_tuple = self.stores.user_friendship.id_tuple(user.id, friend.id);
        let is_1 = id_tuple.0 == user.id;

        if let Some(existing_friendship) = self
            .stores
            .user_friendship
            .find_by_user_ids(user.id, friend.id)
            .await?
        {
            return match existing_friendship.status() {
                FriendshipStatus::RequestedFrom1 => {
                    if is_1 {
                        Err(ServerError::FriendRequestAlreadySent)
                    } else {
                        Ok(self
                            .stores
                            .user_friendship
                            .set_status(user.id, friend.id, FriendshipStatus::RequestedFromBoth)
                            .await?)
                    }
                }
                FriendshipStatus::RequestedFrom2 => {
                    if is_1 {
                        Ok(self
                            .stores
                            .user_friendship
                            .set_status(user.id, friend.id, FriendshipStatus::RequestedFromBoth)
                            .await?)
                    } else {
                        Err(ServerError::FriendRequestAlreadySent)
                    }
                }
                FriendshipStatus::RequestedFromBoth => Err(ServerError::FriendRequestAlreadySent),
                FriendshipStatus::Accepted => Err(ServerError::FriendRequestAlreadyAccepted),
                FriendshipStatus::BlockedBy1 => {
                    if is_1 {
                        Ok(self
                            .stores
                            .user_friendship
                            .set_status(user.id, friend.id, FriendshipStatus::RequestedFrom1)
                            .await?)
                    } else {
                        Err(ServerError::FriendRequestBlocked)
                    }
                }
                FriendshipStatus::BlockedBy2 => {
                    if is_1 {
                        Err(ServerError::FriendRequestBlocked)
                    } else {
                        Ok(self
                            .stores
                            .user_friendship
                            .set_status(user.id, friend.id, FriendshipStatus::RequestedFrom2)
                            .await?)
                    }
                }
                FriendshipStatus::None => Err(ServerError::FriendRequestBlocked),
            };
        }

        let status = if is_1 {
            FriendshipStatus::RequestedFrom1
        } else {
            FriendshipStatus::RequestedFrom2
        };

        let friendship = self
            .stores
            .user_friendship
            .set_status(user.id, friend.id, status)
            .await?;

        Ok(friendship)
    }

    pub async fn accept_request(
        &self,
        user: &user::Model,
        friend_id: String,
    ) -> ServerResult<user_friendship::Model> {
        let friend_uuid = Uuid::parse_str(&friend_id).map_err(|_| ServerError::NoFriendRequest)?;
        let friend = self
            .stores
            .user
            .find_by_id(friend_uuid)
            .await?
            .ok_or(ServerError::NoFriendRequest)?;

        let Some(friendship) = self
            .stores
            .user_friendship
            .find_by_user_ids(user.id, friend.id)
            .await?
        else {
            return Err(ServerError::NoFriendRequest);
        };

        let id_tuple = self.stores.user_friendship.id_tuple(user.id, friend.id);
        let is_1 = id_tuple.0 == user.id;

        match friendship.status() {
            FriendshipStatus::RequestedFrom1 => {
                if is_1 {
                    Err(ServerError::NoFriendRequest)
                } else {
                    Ok(())
                }
            }
            FriendshipStatus::RequestedFrom2 => {
                if is_1 {
                    Ok(())
                } else {
                    Err(ServerError::NoFriendRequest)
                }
            }
            FriendshipStatus::RequestedFromBoth => Ok(()),
            FriendshipStatus::Accepted => Err(ServerError::FriendRequestAlreadyAccepted),
            _ => Err(ServerError::FriendRequestBlocked),
        }?;

        Ok(self
            .stores
            .user_friendship
            .set_status(user.id, friend.id, FriendshipStatus::Accepted)
            .await?)
    }

    pub async fn block_request(&self, user: &user::Model, friend_id: String) -> ServerResult<()> {
        let friend_uuid = Uuid::parse_str(&friend_id).map_err(|_| ServerError::NoFriendRequest)?;
        let friend = self
            .stores
            .user
            .find_by_id(friend_uuid)
            .await?
            .ok_or(ServerError::NoFriendRequest)?;

        let Some(friendship) = self
            .stores
            .user_friendship
            .find_by_user_ids(user.id, friend.id)
            .await?
        else {
            return Err(ServerError::NoFriendRequest);
        };

        let id_tuple = self.stores.user_friendship.id_tuple(user.id, friend.id);
        let is_1 = id_tuple.0 == user.id;

        match friendship.status() {
            FriendshipStatus::RequestedFrom1 => {
                if is_1 {
                    Err(ServerError::NoFriendRequest)
                } else {
                    Ok(())
                }
            }
            FriendshipStatus::RequestedFrom2 => {
                if is_1 {
                    Ok(())
                } else {
                    Err(ServerError::NoFriendRequest)
                }
            }
            FriendshipStatus::RequestedFromBoth => Ok(()),
            FriendshipStatus::Accepted => Err(ServerError::NoFriendRequest),
            _ => Err(ServerError::FriendRequestBlocked),
        }?;

        let status = if is_1 {
            FriendshipStatus::BlockedBy1
        } else {
            FriendshipStatus::BlockedBy2
        };

        self.stores
            .user_friendship
            .set_status(user.id, friend.id, status)
            .await?;

        Ok(())
    }

    pub async fn get_friend_requests(
        &self,
        user: &user::Model,
    ) -> ServerResult<Vec<FriendRequestInfo>> {
        let friendship_requests = self
            .stores
            .user_friendship
            .find_for_user_id_requested(&user.id)
            .await?
            .into_iter()
            .filter(|friendship| friendship.is_requested_from_other(&user.id))
            .collect::<Vec<_>>();

        let mut infos = Vec::new();
        for friendship in friendship_requests {
            let friend_id = friendship.get_other_user_id(&user.id);
            let Some(friend) = self.stores.user.find_by_id(friend_id).await? else {
                continue;
            };
            let info = FriendRequestInfo {
                user_id: friend_id.to_string(),
                user_name: friend.name,
            };
            infos.push(info);
        }

        Ok(infos)
    }

    pub async fn get_friends(&self, user: &user::Model) -> ServerResult<Vec<FriendInfo>> {
        let friendships = self
            .stores
            .user_friendship
            .find_for_user_id_with_status(&user.id, FriendshipStatus::Accepted)
            .await?;

        let mut infos = Vec::new();
        for friendship in friendships {
            let friend_id = friendship.get_other_user_id(&user.id);
            let Some(friend) = self.stores.user.find_by_id(friend_id).await? else {
                continue;
            };
            let info = FriendInfo {
                user_id: friend_id.to_string(),
                user_name: friend.name,
                timestamp_utc: friendship.created_at.and_utc().timestamp(),
                is_online: false,
            };
            infos.push(info);
        }

        Ok(infos)
    }

    pub async fn remove_friend(&self, user: &user::Model, friend_id: String) -> ServerResult<()> {
        let friend_uuid = Uuid::parse_str(&friend_id).map_err(|_| ServerError::NotFriends)?;
        let friendship = self
            .stores
            .user_friendship
            .find_by_user_ids(user.id, friend_uuid)
            .await?
            .ok_or(ServerError::NotFriends)?;

        if friendship.status() != FriendshipStatus::Accepted {
            return Err(ServerError::NotFriends);
        };

        Ok(self.stores.user_friendship.remove(friendship).await?)
    }
}
