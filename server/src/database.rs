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
}

impl ConnectionTrait for Database {
    fn get_database_backend(&self) -> DbBackend {
        self.connection.get_database_backend()
    }

    async fn execute_raw(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        self.connection.execute_raw(stmt).await
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        self.connection.execute_unprepared(sql).await
    }

    async fn query_one_raw(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        self.connection.query_one_raw(stmt).await
    }

    async fn query_all_raw(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        self.connection.query_all_raw(stmt).await
    }
}
