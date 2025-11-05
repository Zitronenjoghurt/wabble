use crate::database::entity::invite_code;
use crate::database::Database;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait};
use std::sync::Arc;

pub struct InviteCodeStore {
    db: Arc<Database>,
}

impl InviteCodeStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn create_new(&self) -> anyhow::Result<invite_code::Model> {
        let active_model = invite_code::ActiveModel::new();
        Ok(active_model.insert(self.db.as_ref()).await?)
    }

    pub async fn find_random(&self) -> anyhow::Result<Option<invite_code::Model>> {
        Ok(invite_code::Entity::find().one(self.db.as_ref()).await?)
    }
}
