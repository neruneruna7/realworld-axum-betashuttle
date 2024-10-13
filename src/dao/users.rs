use crate::{
    core::users::{dao_trait::UsersDaoTrait, dto::PasswdHashedNewUser, entity::UserEntity},
    error::ConduitResult,
};
use anyhow::Context as _;
use axum::async_trait;
use sqlx::{Executor, PgConnection};
use uuid::Uuid;

use super::db_client::DbClient;

#[async_trait]
impl UsersDaoTrait for DbClient
// where
//     CN: Executor<'_, Database = sqlx::Postgres> + Send + Sync,
{
    type Connection = PgConnection;
    /// パスワードをハッシュ化しないまま値を渡さないでください
    async fn create_user(
        &self,
        conn: &mut Self::Connection,
        user_hashed_password: PasswdHashedNewUser,
    ) -> ConduitResult<UserEntity> {
        // UUIDを生成する
        let uuid = Uuid::now_v7();

        let user = sqlx::query_as!(
            UserEntity,
            r#"
            INSERT INTO users (id, username, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING username, email, bio, image, id, created_at, updated_at, password
            "#,
            uuid,
            user_hashed_password.username,
            user_hashed_password.email,
            user_hashed_password.password
        )
        .fetch_one(&mut *conn)
        .await
        .context("unexpected error: while inserting user")?;

        // // トランザクションがSomeの場合はトランザクションを使う
        // let user = self
        //     .execute_query(query)
        //     .await
        //     .context("unexpected error: while inserting user")?;

        Ok(user)
    }

    async fn get_user_by_id(&mut self, user_id: Uuid) -> ConduitResult<UserEntity> {
        let query = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            user_id
        );
        let user = self
            .execute_query(query)
            .await
            .context("unexpected error: while fetching user")?;
        Ok(user)
    }

    async fn get_user_by_email(&mut self, email: &str) -> ConduitResult<Option<UserEntity>> {
        let query = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
            email
        );
        let user = self
            .execute_query_optional(query)
            .await
            .context("unexpected error: while querying for user by email")?;
        Ok(user)
    }

    async fn get_user_by_username(&mut self, username: &str) -> ConduitResult<Option<UserEntity>> {
        let query = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT *
            FROM users
            WHERE username = $1
            "#,
            username
        );
        let user = self
            .execute_query_optional(query)
            .await
            .context("unexpected error: while querying for user by username")?;
        Ok(user)
    }

    async fn update_user(&mut self, user: UserEntity) -> ConduitResult<UserEntity> {
        let q = sqlx::query_as!(
            UserEntity,
            r#"
            UPDATE users
            SET username = $1, email = $2, bio = $3, image = $4, password = $5
            WHERE id = $6
            RETURNING *
            "#,
            user.username,
            user.email,
            user.bio,
            user.image,
            user.password,
            user.id
        );
        let user = self.execute_query(q).await.context("failed: user update")?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::users::entity::UserEntity;
    use sqlx::PgPool;

    #[sqlx::test()]
    async fn test_create_user(pool: PgPool) {
        let new_user = PasswdHashedNewUser {
            email: "test@gmail.com".to_string(),
            password: "password".to_string(),
            username: "test".to_string(),
        };
        let mut dao = DbClient::new(pool.clone());
        let mut tx = pool.begin().await.unwrap();
        let user = dao.create_user(&mut *tx, new_user.clone()).await.unwrap();
        let test_user = UserEntity {
            id: user.id,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: new_user.username,
            email: new_user.email,
            password: new_user.password,
            bio: "".to_string(),
            image: None,
        };
        assert_eq!(user, test_user);
    }

    #[sqlx::test()]
    async fn test_get_user_by_id(pool: PgPool) {
        let new_user = PasswdHashedNewUser {
            email: "userid_get_test@gmail.com".to_string(),
            password: "password".to_string(),
            username: "userid_get_test".to_string(),
        };
        let mut dao = DbClient::new(pool.clone());
        let mut p = pool.acquire().await.unwrap();
        let user = dao.create_user(&mut *p, new_user.clone()).await.unwrap();
        let get_user = dao.get_user_by_id(user.id).await.unwrap();

        assert_eq!(user, get_user);
    }

    #[sqlx::test()]
    async fn test_get_user_by_email(pool: PgPool) {
        let new_user = PasswdHashedNewUser {
            email: "email_get_test@gmail.com".to_string(),
            password: "password".to_string(),
            username: "email_get_test".to_string(),
        };
        let mut dao = DbClient::new(pool);
        let user = dao.create_user(new_user.clone()).await.unwrap();
        let get_user = dao.get_user_by_email(&new_user.email).await.unwrap();
        assert_eq!(user, get_user.unwrap());
    }
}
