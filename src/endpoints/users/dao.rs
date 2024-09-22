use super::dto::{NewUser, User};
use crate::{endpoints::users::entity::UserEntity, error::ConduitResult};
use sqlx::Executor;

pub struct PasswdHashedNewUser {
    username: String,
    email: String,
    password: String,
}

impl PasswdHashedNewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}

pub struct UserDao {
    pool: sqlx::PgPool,
}

impl UserDao {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// パスワードをハッシュ化しないまま値を渡さないでください
    pub async fn create_user(
        &self,
        user_hashed_password: PasswdHashedNewUser,
    ) -> ConduitResult<UserEntity> {
        // let mut conn = self.pool.acquire().await?;
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING username, email, bio, image, id, created_at, updated_at, password
            "#,
            user_hashed_password.username,
            user_hashed_password.email,
            user_hashed_password.password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
