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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dao::users::UserDao,
        endpoints::users::{dao_trait::UsersDaoTrait, dto::NewUser, entity::UserEntity},
        services::hash::PasswordHashService,
    };
    use sqlx::PgPool;

    async fn setup_user_ab(pool: &PgPool) -> (UserEntity, UserEntity) {
        let user_dao = UserDao::new(pool.clone());
        // テスト用のユーザーAとBを作成
        let new_a_user = NewUser {
            username: Some("test_user_a".to_string()),
            email: Some("test@example.com".to_string()),
            password: Some("password".to_string()),
        };
        let new_a_user = PasswordHashService::hash_password_newuser(new_a_user).unwrap();
        let user_a = user_dao.create_user(new_a_user).await.unwrap();

        let new_b_user = NewUser {
            username: Some("test_user_b".to_string()),
            email: Some("testb@eexample.com".to_string()),
            password: Some("password".to_string()),
        };
        let new_b_user = PasswordHashService::hash_password_newuser(new_b_user).unwrap();
        let user_b = user_dao.create_user(new_b_user).await.unwrap();
        (user_a, user_b)
    }

    #[sqlx::test]
    async fn test_following_user(pool: PgPool) {
        // テスト用のユーザーAとBを作成
        let (user_a, user_b) = setup_user_ab(&pool).await;

        // ユーザーAがユーザーBをフォローする
        let profile_dao = ProfileDao::new(pool.clone());
        let user_follow = profile_dao
            .following_user(user_a.id, user_b.id)
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
        let profile_dao = ProfileDao::new(pool.clone());
        let user_follow = profile_dao
            .following_user(user_a.id, user_b.id)
            .await
            .unwrap();
        assert_eq!(user_follow.follower_id, user_a.id);
        assert_eq!(user_follow.followee_id, user_b.id);

        // ユーザーAがユーザーBのフォローを解除する
        let user_follow = profile_dao
            .unfollow_user(user_a.id, user_b.id)
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
        let profile_dao = ProfileDao::new(pool.clone());
        let user_follow = profile_dao
            .following_user(user_a.id, user_b.id)
            .await
            .unwrap();
        assert_eq!(user_follow.follower_id, user_a.id);
        assert_eq!(user_follow.followee_id, user_b.id);

        // ユーザーAがフォローしているユーザーを取得
        let user_follows = profile_dao.get_user_followees(user_a.id).await.unwrap();
        assert_eq!(user_follows.len(), 1);
        assert_eq!(user_follows[0].follower_id, user_a.id);
        assert_eq!(user_follows[0].followee_id, user_b.id);
    }
}
