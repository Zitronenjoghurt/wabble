use crate::database::entity::user;
use crate::database::Database;
use crate::stores::{StoreError, StoreResult};
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ColumnTrait, Set};
use sea_orm::{EntityTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;
use wabble_core::types::user_permissions::UserPermissions;

pub struct UserStore {
    db: Arc<Database>,
}

impl UserStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub async fn find_by_id(&self, id: Uuid) -> StoreResult<Option<user::Model>> {
        Ok(user::Entity::find()
            .filter(user::Column::Id.eq(id))
            .one(self.db.conn())
            .await?)
    }

    pub async fn find_by_username(&self, username: &str) -> StoreResult<Option<user::Model>> {
        Ok(user::Entity::find()
            .filter(user::Column::Name.eq(username.to_ascii_lowercase()))
            .one(self.db.conn())
            .await?)
    }

    pub async fn find_by_friend_code(&self, friend_code: &str) -> StoreResult<Option<user::Model>> {
        Ok(user::Entity::find()
            .filter(user::Column::FriendCode.eq(friend_code))
            .one(self.db.conn())
            .await?)
    }

    pub async fn create_new(
        &self,
        username: &str,
        password_hash: &str,
        invite_code: &Uuid,
    ) -> StoreResult<user::Model> {
        let existing_user = self.find_by_username(username).await?;
        if existing_user.is_some() {
            return Err(StoreError::UserAlreadyExists);
        }

        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            invite_code: Set(*invite_code),
            name: Set(username.to_ascii_lowercase()),
            password_hash: Set(password_hash.to_string()),
            permissions: Set(UserPermissions::default().bits()),
            friend_code: Set(nanoid!(12)),
            ..Default::default()
        };
        Ok(new_user.insert(self.db.conn()).await?)
    }
}
