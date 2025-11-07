use crate::database::entity::invite_code;
use crate::database::Database;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, Set};
use std::sync::Arc;
use uuid::Uuid;

pub struct InviteCodeStore {
    db: Arc<Database>,
}

impl InviteCodeStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn create_new(&self) -> anyhow::Result<invite_code::Model> {
        let active_model = invite_code::ActiveModel {
            code: Set(Uuid::new_v4()),
            ..Default::default()
        };
        Ok(active_model.insert(self.db.conn()).await?)
    }

    pub async fn create_many(&self, amount: u8) -> anyhow::Result<()> {
        for _ in 0..amount {
            self.create_new().await?;
        }
        Ok(())
    }

    pub async fn find_random(&self) -> anyhow::Result<Option<invite_code::Model>> {
        Ok(invite_code::Entity::find().one(self.db.conn()).await?)
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<invite_code::Model>> {
        Ok(invite_code::Entity::find().all(self.db.conn()).await?)
    }
}
