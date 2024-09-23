use sqlx::prelude::*;
use sqlx::types::time::PrimitiveDateTime;
use uuid::Uuid;

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
