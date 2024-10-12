use crate::{
    core::profiles::{dao_trait::ProfilesDaoTrait, entity::UserFollowEntity},
    error::ConduitResult,
};
use anyhow::Context as _;
use axum::async_trait;
use sqlx::{PgExecutor, PgPool};
use uuid::Uuid;

use super::db_client::DbClient;

#[async_trait]
impl ProfilesDaoTrait for DbClient {
    type Connection = PgPool;
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
        conn: &Self::Connection,
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
        conn: &Self::Connection,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::users::{dao_trait::UsersDaoTrait, dto::NewUser, entity::UserEntity},
        services::hash::PasswordHashService,
    };
    use sqlx::PgPool;

    async fn setup_user_ab(pool: &PgPool) -> (UserEntity, UserEntity) {
        let db_client = DbClient::new(pool.clone());
        // テスト用のユーザーAとBを作成
        let new_a_user = NewUser {
            username: Some("test_user_a".to_string()),
            email: Some("test@example.com".to_string()),
            password: Some("password".to_string()),
        };
        let new_a_user = PasswordHashService::hash_password_newuser(new_a_user).unwrap();
        let user_a = db_client.create_user(new_a_user).await.unwrap();

        let new_b_user = NewUser {
            username: Some("test_user_b".to_string()),
            email: Some("testb@eexample.com".to_string()),
            password: Some("password".to_string()),
        };
        let new_b_user = PasswordHashService::hash_password_newuser(new_b_user).unwrap();
        let user_b = db_client.create_user(new_b_user).await.unwrap();
        (user_a, user_b)
    }

    #[sqlx::test]
    async fn test_following_user(pool: PgPool) {
        // テスト用のユーザーAとBを作成
        let (user_a, user_b) = setup_user_ab(&pool).await;

        // ユーザーAがユーザーBをフォローする
        let db_client = DbClient::new(pool.clone());
        let user_follow = db_client
            .following_user(&pool, user_a.id, user_b.id)
            .await
            .unwrap();
        assert_eq!(user_follow.follower_id, user_a.id);
        assert_eq!(user_follow.followee_id, user_b.id);
    }

    #[sqlx::test]
    async fn test_unfollow_user(pool: PgPool) {
        // テスト用のユーザーAとBを作成
        let (user_a, user_b) = setup_user_ab(&pool).await;

        // ユーザーAがユーザーBをフォローする
        let db_client = DbClient::new(pool.clone());
        let user_follow = db_client
            .following_user(&pool, user_a.id, user_b.id)
            .await
            .unwrap();
        assert_eq!(user_follow.follower_id, user_a.id);
        assert_eq!(user_follow.followee_id, user_b.id);

        // ユーザーAがユーザーBのフォローを解除する
        let user_follow = db_client
            .unfollow_user(&pool, user_a.id, user_b.id)
            .await
            .unwrap();
        assert_eq!(user_follow.follower_id, user_a.id);
        assert_eq!(user_follow.followee_id, user_b.id);
    }

    #[sqlx::test]
    async fn test_get_user_followees(pool: PgPool) {
        // テスト用のユーザーAとBを作成
        let (user_a, user_b) = setup_user_ab(&pool).await;

        // ユーザーAがユーザーBをフォローする
        let dbclient = DbClient::new(pool.clone());
        let user_follow = dbclient
            .following_user(&pool, user_a.id, user_b.id)
            .await
            .unwrap();
        assert_eq!(user_follow.follower_id, user_a.id);
        assert_eq!(user_follow.followee_id, user_b.id);

        // ユーザーAがフォローしているユーザーを取得
        let user_follows = dbclient.get_user_followees(user_a.id).await.unwrap();
        assert_eq!(user_follows.len(), 1);
        assert_eq!(user_follows[0].follower_id, user_a.id);
        assert_eq!(user_follows[0].followee_id, user_b.id);
    }
}
