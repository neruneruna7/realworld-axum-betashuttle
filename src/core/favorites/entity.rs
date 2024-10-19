use sqlx::{prelude::FromRow, types::time::PrimitiveDateTime};
use uuid::Uuid;

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct FavoritesEntity {
    pub user_id: Uuid,
    pub article_id: i32,
    pub created_at: PrimitiveDateTime,
    pub is_deleted: bool,
}
