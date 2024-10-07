use crate::{
    endpoints::profiles::{dao_trait::ProfilesDaoTrait, entity::UserFollowEntity},
    error::ConduitResult,
};
use anyhow::Context as _;
use axum::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProfileDao {
    pool: sqlx::PgPool,
}

impl ProfileDao {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfilesDaoTrait for ProfileDao {
    /// ユーザーIDをもつユーザーがフォローしているユーザーリストを取得
    async fn get_user_followees(
        &self,
        follower_user_id: Uuid,
    ) -> ConduitResult<Vec<UserFollowEntity>> {
        let user_follows = sqlx::query_as!(
            UserFollowEntity,
            r#"
            SELECT id, created_at, follower_id, followee_id
            FROM user_follows
            WHERE follower_id = $1
            "#,
            follower_user_id
        )
        .fetch_all(&self.pool)
        .await
        .context("unexpected error: while fetching user_follows")?;
        Ok(user_follows)
    }

    async fn following_user(
        &self,
        follower_id: Uuid,
        followee_id: Uuid,
    ) -> ConduitResult<UserFollowEntity> {
        let uuid = Uuid::now_v7();
        let user_follow = sqlx::query_as!(
            UserFollowEntity,
            r#"
            INSERT INTO user_follows (id, follower_id, followee_id)
            VALUES ($1, $2, $3)
            RETURNING id, created_at, follower_id, followee_id
            "#,
            uuid,
            follower_id,
            followee_id
        )
        .fetch_one(&self.pool)
        .await
        .context("unexpected error: while inserting user_follow")?;
        Ok(user_follow)
    }

    async fn unfollow_user(
        &self,
        follower_id: Uuid,
        followee_id: Uuid,
    ) -> ConduitResult<UserFollowEntity> {
        let user_follow = sqlx::query_as!(
            UserFollowEntity,
            r#"
            DELETE FROM user_follows
            WHERE follower_id = $1 AND followee_id = $2
            RETURNING id, created_at, follower_id, followee_id
            "#,
            follower_id,
            followee_id
        )
        .fetch_one(&self.pool)
        .await
        .context("unexpected error: while deleting user_follow")?;
        Ok(user_follow)
    }
}
