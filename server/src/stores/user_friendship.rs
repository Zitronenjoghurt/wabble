use crate::database::entity::user_friendship;
use crate::database::Database;
use crate::stores::StoreResult;
use sea_orm::ColumnTrait;
use sea_orm::{
    ActiveModelTrait, EntityTrait, ExprTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
};
use std::sync::Arc;
use uuid::Uuid;
use wabble_core::types::friendship_status::FriendshipStatus;

pub struct UserFriendshipStore {
    db: Arc<Database>,
}

impl UserFriendshipStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    pub fn id_tuple(&self, user_1_id: Uuid, user_2_id: Uuid) -> (Uuid, Uuid) {
        if user_1_id < user_2_id {
            (user_1_id, user_2_id)
        } else {
            (user_2_id, user_1_id)
        }
    }

    pub async fn find_by_user_ids(
        &self,
        user_1_id: Uuid,
        user_2_id: Uuid,
    ) -> StoreResult<Option<user_friendship::Model>> {
        let id = self.id_tuple(user_1_id, user_2_id);
        Ok(user_friendship::Entity::find_by_id(id)
            .one(self.db.conn())
            .await?)
    }

    pub async fn find_for_user_id(
        &self,
        user_id: Uuid,
    ) -> StoreResult<Vec<user_friendship::Model>> {
        Ok(user_friendship::Entity::find()
            .filter(
                user_friendship::Column::User1Id
                    .eq(user_id)
                    .or(user_friendship::Column::User2Id.eq(user_id)),
            )
            .all(self.db.conn())
            .await?)
    }

    pub async fn find_for_user_id_with_status(
        &self,
        user_id: Uuid,
        status: FriendshipStatus,
    ) -> StoreResult<Vec<user_friendship::Model>> {
        Ok(user_friendship::Entity::find()
            .filter(
                user_friendship::Column::User1Id
                    .eq(user_id)
                    .or(user_friendship::Column::User2Id.eq(user_id)),
            )
            .filter(user_friendship::Column::Status.eq::<i16>(status.into()))
            .all(self.db.conn())
            .await?)
    }

    pub async fn set_status(
        &self,
        user_1_id: Uuid,
        user_2_id: Uuid,
        status: FriendshipStatus,
    ) -> StoreResult<()> {
        if let Some(existing_friendship) = self.find_by_user_ids(user_1_id, user_2_id).await? {
            let mut active_model = existing_friendship.into_active_model();
            active_model.status = Set(status.into());
            active_model.update(self.db.conn()).await?;
        } else {
            let id_tuple = self.id_tuple(user_1_id, user_2_id);
            let new_friendship = user_friendship::ActiveModel {
                user1_id: Set(id_tuple.0),
                user2_id: Set(id_tuple.1),
                status: Set(status.into()),
                ..Default::default()
            };
            new_friendship.insert(self.db.conn()).await?;
        }

        Ok(())
    }

    pub async fn remove(&self, user_1_id: Uuid, user_2_id: Uuid) -> StoreResult<()> {
        let Some(friendship) = self.find_by_user_ids(user_1_id, user_2_id).await? else {
            return Ok(());
        };
        friendship.delete(self.db.conn()).await?;
        Ok(())
    }
}
