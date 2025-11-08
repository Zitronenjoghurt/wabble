use crate::database::entity::user_session;
use crate::database::Database;
use crate::stores::StoreResult;
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use std::sync::Arc;
use uuid::Uuid;

pub struct UserSessionStore {
    db: Arc<Database>,
}

impl UserSessionStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn delete(&self, user_id: Uuid) -> StoreResult<()> {
        if let Some(session) = user_session::Entity::find_by_id(user_id)
            .one(self.db.conn())
            .await?
        {
            session.delete(self.db.conn()).await?;
        };
        Ok(())
    }

    pub async fn create_new(
        &self,
        user_id: Uuid,
        password_hash: String,
        expires_at: NaiveDateTime,
    ) -> StoreResult<user_session::Model> {
        self.delete(user_id).await?;

        let new_session = user_session::ActiveModel {
            user_id: Set(user_id),
            token_hash: Set(password_hash),
            expires_at: Set(expires_at),
            ..Default::default()
        };

        Ok(new_session.insert(self.db.conn()).await?)
    }

    pub async fn find(&self, user_id: Uuid) -> StoreResult<Option<user_session::Model>> {
        if let Some(session) = user_session::Entity::find_by_id(user_id)
            .one(self.db.conn())
            .await?
        {
            let now = chrono::Utc::now().naive_utc();
            if now > session.expires_at {
                self.delete(user_id).await?;
                return Ok(None);
            }

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }
}
