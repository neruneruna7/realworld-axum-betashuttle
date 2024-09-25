use crate::{endpoints::users::entity::UserEntity, error::ConduitResult};
use anyhow::Context as _;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PasswdHashedNewUser {
    pub username: String,
    pub email: String,
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
        .fetch_one(&self.pool)
        .await
        .context("unexpect error: while inserting user")?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> ConduitResult<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("user not found")?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoints::users::entity::UserEntity;
    use sqlx::PgPool;

    #[sqlx::test()]
    async fn test_create_user(pool: PgPool) {
        let new_user = PasswdHashedNewUser {
            email: "test@gmail.com".to_string(),
            password: "password".to_string(),
            username: "test".to_string(),
        };
        let dao = UserDao::new(pool);
        let user = dao.create_user(new_user.clone()).await.unwrap();
        let test_user = UserEntity {
            id: user.id,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: new_user.username,
            email: new_user.email,
            password: new_user.password,
            bio: "".to_string(),
            image: "".to_string(),
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
        let dao = UserDao::new(pool);
        let user = dao.create_user(new_user.clone()).await.unwrap();
        let get_user = dao.get_user_by_id(user.id).await.unwrap();

        assert_eq!(user, get_user);
    }
}
