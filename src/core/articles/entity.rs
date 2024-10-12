use sqlx::{prelude::FromRow, types::time::PrimitiveDateTime};
use uuid::Uuid;

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct ArticleEntity {
    pub id: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub author_id: Uuid,
}
