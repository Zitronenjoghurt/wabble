use crate::database::entity::user;
use crate::database::Database;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;

pub struct UserStore {
    db: Arc<Database>,
}

impl UserStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn fetch_by_id(&self, id: Uuid) -> anyhow::Result<Option<user::Model>> {
        Ok(user::Entity::find()
            .filter(user::Column::Id.eq(id))
            .one(self.db.conn())
            .await?)
    }

    pub async fn fetch_by_username(&self, username: &str) -> anyhow::Result<Option<user::Model>> {
        Ok(user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .one(self.db.conn())
            .await?)
    }
}
