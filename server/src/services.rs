use crate::stores::Stores;
use std::sync::Arc;

pub mod user;

pub struct Services {
    pub user: Arc<user::UserService>,
}

impl Services {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            user: user::UserService::initialize(stores),
        })
    }
}
