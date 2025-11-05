use crate::config::Config;
use crate::database::Database;
use crate::services::Services;
use crate::stores::Stores;
use crate::websocket::connections::ConnectionRegistry;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub config: Arc<Config>,
    pub stores: Arc<Stores>,
    pub services: Arc<Services>,
    pub connections: Arc<ConnectionRegistry>,
}

impl ServerState {
    pub async fn initialize() -> anyhow::Result<Arc<Self>> {
        let config = Config::load_from_env()?;
        let db = Database::initialize(&config.db_url).await?;
        let stores = Stores::initialize(&db);
        let services = Services::initialize(&stores);
        let connections = ConnectionRegistry::initialize();
        Ok(Arc::new(Self {
            config,
            stores,
            services,
            connections,
        }))
    }
}
