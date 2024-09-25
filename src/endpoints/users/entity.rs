use sqlx::prelude::*;
use sqlx::types::time::PrimitiveDateTime;
use uuid::Uuid;

use super::dto::User;

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct UserEntity {
    pub id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl UserEntity {
    pub fn into_dto_with_generated_token(self, token: String) -> User {
        User {
            email: self.email,
            username: self.username,
            bio: self.bio,
            image: Some(self.image),
            token,
        }
    }
}
