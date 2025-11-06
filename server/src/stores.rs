use crate::database::Database;
use std::sync::Arc;

pub mod invite_code;
pub mod user;

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
