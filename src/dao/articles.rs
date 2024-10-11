use anyhow::Context as _;
use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    endpoints::articles::{
        dao_trait::ArticlesDaoTrait,
        dto::{Article, NewArticle},
        entity::ArticleEntity,
    },
    error::ConduitError,
};

#[derive(Clone)]
pub struct ArticlesDao {
    pool: PgPool,
}

impl ArticlesDao {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticlesDaoTrait for ArticlesDao {
    async fn create_article(
        &self,
        article: NewArticle,
        author_id: Uuid,
    ) -> Result<ArticleEntity, ConduitError> {
        let article = sqlx::query_as!(
            ArticleEntity,
            r#"
            INSERT INTO articles (author_id, title, description, body)
            VALUES ($1, $2, $3, $4)
            RETURNING id, author_id, title, description, body, created_at, updated_at
            "#,
            author_id,
            article.title,
            article.description,
            article.body
        )
        .fetch_one(&self.pool)
        .await
        .context("unexpected error: while inserting article")?;
        Ok(article)
    }
}
