use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ConnectOptions, ConnectionTrait, DatabaseConnection, DbBackend, DbErr, ExecResult, QueryResult,
    Statement,
};
use std::sync::Arc;

pub mod entity;

pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn initialize(url: impl Into<String>) -> anyhow::Result<Arc<Self>> {
        let options = ConnectOptions::new(url);

        info!("Connecting to database...");
        let connection = sea_orm::Database::connect(options).await?;
        info!("Database connection established");

        info!("Applying database migrations...");
        Migrator::up(&connection, None).await?;
        info!("Database migrations applied");

        let db = Self { connection };
        Ok(Arc::new(db))
    }

    pub fn conn(&self) -> &DatabaseConnection {
        &self.connection
    }
}
