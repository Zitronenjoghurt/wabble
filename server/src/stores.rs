use crate::database::Database;
use log::error;
use std::sync::Arc;
use wabble_core::message::server::ServerError;

pub mod invite_code;
pub mod user;
pub mod user_friendship;
pub mod user_session;

pub type StoreResult<T> = Result<T, StoreError>;
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),
    #[error("User already exists")]
    UserAlreadyExists,
}

impl From<StoreError> for ServerError {
    fn from(value: StoreError) -> Self {
        match value {
            StoreError::Database(err) => {
                error!("Database error: {err}");
                ServerError::Database
            }
            StoreError::UserAlreadyExists => ServerError::UsernameTaken,
        }
    }
}

pub struct Stores {
    pub invite_code: Arc<invite_code::InviteCodeStore>,
    pub user: Arc<user::UserStore>,
    pub user_friendship: Arc<user_friendship::UserFriendshipStore>,
    pub user_session: Arc<user_session::UserSessionStore>,
}

impl Stores {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            invite_code: invite_code::InviteCodeStore::initialize(db),
            user: user::UserStore::initialize(db),
            user_friendship: user_friendship::UserFriendshipStore::initialize(db),
            user_session: user_session::UserSessionStore::initialize(db),
        })
    }
}
