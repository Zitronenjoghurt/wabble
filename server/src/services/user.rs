use crate::crypto::hash_password;
use crate::crypto::secret::Secret;
use crate::database::entity::user;
use crate::stores::Stores;
use std::sync::Arc;
use uuid::Uuid;
use wabble_core::message::server::{ServerError, ServerResult};
use wabble_core::validate::{validate_password, validate_username};

pub struct UserService {
    stores: Arc<Stores>,
}

impl UserService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub async fn register_user(
        &self,
        username: String,
        password: Secret,
        code_string: String,
    ) -> ServerResult<user::Model> {
        validate_username(&username)?;
        validate_password(password.reveal_str())?;

        let password_hash = hash_password(&password)?;
        drop(password);

        let code_uuid =
            Uuid::parse_str(&code_string).map_err(|_| ServerError::InvalidInviteCode)?;
        let Some(invite_code) = self.stores.invite_code.find_by_code(code_uuid).await? else {
            return Err(ServerError::InvalidInviteCode);
        };

        let new_user = self
            .stores
            .user
            .create_new(&username, &password_hash, &invite_code.code)
            .await?;
        self.stores.invite_code.delete(invite_code).await?;

        Ok(new_user)
    }
}
