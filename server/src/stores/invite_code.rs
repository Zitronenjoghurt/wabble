use crate::database::entity::invite_code;
use crate::database::Database;
use crate::stores::StoreResult;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use std::sync::Arc;
use uuid::Uuid;

pub struct InviteCodeStore {
    db: Arc<Database>,
}

impl InviteCodeStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn delete(&self, invite_code: invite_code::Model) -> StoreResult<()> {
        invite_code.delete(self.db.conn()).await?;
        Ok(())
    }

    pub async fn create_new(&self) -> StoreResult<invite_code::Model> {
        let active_model = invite_code::ActiveModel {
            code: Set(Uuid::new_v4()),
            ..Default::default()
        };
        Ok(active_model.insert(self.db.conn()).await?)
    }

    pub async fn create_many(&self, amount: u8) -> StoreResult<()> {
        for _ in 0..amount {
            self.create_new().await?;
        }
        Ok(())
    }

    pub async fn find_all(&self) -> StoreResult<Vec<invite_code::Model>> {
        Ok(invite_code::Entity::find().all(self.db.conn()).await?)
    }

    pub async fn find_by_code(&self, code: Uuid) -> StoreResult<Option<invite_code::Model>> {
        Ok(invite_code::Entity::find_by_id(code)
            .one(self.db.conn())
            .await?)
    }
}
