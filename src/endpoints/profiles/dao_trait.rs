use axum::async_trait;
use uuid::Uuid;

use crate::error::ConduitResult;

use super::entity::UserFollowEntity;

#[async_trait]
pub trait ProfilesDao {
    async fn get_user_following(&self, user_id: Uuid) -> ConduitResult<Vec<UserFollowEntity>>;
    async fn following_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> ConduitResult<UserFollowEntity>;
}
