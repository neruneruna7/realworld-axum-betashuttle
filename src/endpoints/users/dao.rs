use crate::error::ConduitResult;

use super::dto::{NewUser, User};

pub struct UserDao {
    pool: sqlx::PgPool,
}

impl UserDao {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: NewUser) -> ConduitResult<User> {
        let mut conn = self.pool.acquire().await?;
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING username, email, bio, image
            "#,
            user.username,
            user.email,
            user.password
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(user)
    }
}
