use crate::stores::Stores;
use std::sync::Arc;

pub mod friendship;
pub mod user;

pub struct Services {
    pub friendship: Arc<friendship::FriendshipService>,
    pub user: Arc<user::UserService>,
}

impl Services {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            friendship: friendship::FriendshipService::initialize(stores),
            user: user::UserService::initialize(stores),
        })
    }
}
