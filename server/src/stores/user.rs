use crate::database::Database;
use std::sync::Arc;

pub struct UserStore {
    db: Arc<Database>,
}

impl UserStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }
}
