use crate::config::Config;
use crate::crypto::hash_password;
use anyhow::Context;
use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::sync::Arc;
use wabble_core::types::user_permissions::UserPermissions;

pub mod entity;
mod entity_ext;

pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn initialize(config: &Arc<Config>) -> anyhow::Result<Arc<Self>> {
        let options = ConnectOptions::new::<&str>(config.db_url.as_ref());

        info!("Connecting to database...");
        let connection = sea_orm::Database::connect(options).await?;
        info!("Database connection established");

        let db = Self { connection };
        db.apply_migrations().await?;
        db.init_admin_user(config).await?;

        Ok(Arc::new(db))
    }

    pub fn conn(&self) -> &DatabaseConnection {
        &self.connection
    }

    async fn apply_migrations(&self) -> anyhow::Result<()> {
        info!("Applying database migrations...");
        Migrator::up(&self.connection, None).await?;
        info!("Database migrations applied");
        Ok(())
    }

    async fn init_admin_user(&self, config: &Arc<Config>) -> anyhow::Result<()> {
        if entity::user::Entity::find()
            .filter(entity::user::Column::Name.eq(&config.admin_user))
            .one(&self.connection)
            .await?
            .is_some()
        {
            info!("Admin user already exists");
            return Ok(());
        };

        let password_hash =
            hash_password(&config.admin_token).context("Failed to hash admin password")?;
        let new_user = entity::user::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            name: Set(config.admin_user.to_string()),
            password_hash: Set(password_hash),
            permissions: Set(UserPermissions::ADMIN.bits()),
            ..Default::default()
        };

        new_user.insert(&self.connection).await?;
        info!("Admin user created");

        Ok(())
    }
}
