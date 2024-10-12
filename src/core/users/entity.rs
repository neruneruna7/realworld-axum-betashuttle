use sqlx::prelude::*;
use sqlx::types::time::PrimitiveDateTime;
use uuid::Uuid;

use super::dto::{UpdateUser, User};

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct UserEntity {
    pub id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: Option<String>,
}

impl UserEntity {
    pub fn into_dto_with_generated_token(self, token: String) -> User {
        User {
            email: self.email,
            username: self.username,
            bio: self.bio,
            image: self.image,
            token,
        }
    }
    pub(crate) fn update_user_entity(self, update_user: UpdateUser) -> Self {
        UserEntity {
            email: update_user.email.unwrap_or(self.email),
            password: update_user.password.unwrap_or(self.password),
            username: update_user.username.unwrap_or(self.username),
            bio: update_user.bio.unwrap_or(self.bio),
            image: update_user.image.or(self.image),
            ..self
        }
    }
}
