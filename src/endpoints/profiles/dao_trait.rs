use std::sync::Arc;

use axum::async_trait;
use uuid::Uuid;

use crate::error::ConduitResult;

use super::entity::UserFollowEntity;

pub type DynProfilesDao = Arc<dyn ProfilesDaoTrait + Send + Sync>;

// Traitなのか、Structとかなのか、命名を見ただけじゃはっきりしないことがある
// ので，あえて命名にTraitを含めている
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProfilesDaoTrait {
    async fn get_user_followees(&self, user_id: Uuid) -> ConduitResult<Vec<UserFollowEntity>>;
    async fn following_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> ConduitResult<UserFollowEntity>;

    async fn unfollow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> ConduitResult<UserFollowEntity>;
}
