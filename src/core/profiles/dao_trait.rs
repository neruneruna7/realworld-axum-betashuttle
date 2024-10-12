use std::sync::Arc;

use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ConduitResult;

use super::entity::UserFollowEntity;

// Traitなのか、Structとかなのか、命名を見ただけじゃはっきりしないことがある
// ので，あえて命名にTraitを含めている
// #[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProfilesDaoTrait {
    type Connection;
    async fn get_user_followees(&self, user_id: Uuid) -> ConduitResult<Vec<UserFollowEntity>>;
    async fn following_user(
        &self,
        conn: &Self::Connection,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> ConduitResult<UserFollowEntity>;

    async fn unfollow_user(
        &self,
        conn: &Self::Connection,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> ConduitResult<UserFollowEntity>;
}
