use sqlx::{prelude::FromRow, types::time::PrimitiveDateTime};

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct TagEntity {
    pub id: i32,
    pub tag: String,
    pub created_at: PrimitiveDateTime,
}

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct ArticleTagEntity {
    pub article_id: i32,
    pub tag_id: i32,
    pub created_at: PrimitiveDateTime,
}

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct ArticleTagQuery {
    pub article_id: i32,
    pub tag_id: i32,
    pub tag: String,
}
