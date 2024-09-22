use sqlx::prelude::*;
use sqlx::types::time::{OffsetDateTime, PrimitiveDateTime};

#[derive(FromRow)]
pub struct UserEntity {
    pub id: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}
