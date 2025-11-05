use crate::stores::Stores;
use std::sync::Arc;

pub struct UserService {
    stores: Arc<Stores>,
}

impl UserService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }
}
