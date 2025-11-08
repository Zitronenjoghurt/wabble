use crate::database::Database;
use log::error;
use std::sync::Arc;
use wabble_core::message::server::ServerError;

pub mod invite_code;
pub mod user;

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
}

impl Stores {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            invite_code: invite_code::InviteCodeStore::initialize(db),
            user: user::UserStore::initialize(db),
        })
    }
}
