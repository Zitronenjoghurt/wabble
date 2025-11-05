use crate::database::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    db: Arc<Database>,
}

impl ServerState {
    pub async fn initialize() -> anyhow::Result<Arc<Self>> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = Database::initialize(db_url).await?;
        Ok(Arc::new(Self { db }))
    }
}
