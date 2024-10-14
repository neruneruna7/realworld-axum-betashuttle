use std::sync::Arc;

use crate::error::{ConduitError, ConduitResult};
use axum::async_trait;
use uuid::Uuid;

use super::dto::{NewArticleValidated, UpdateArticle};
use super::entity::ArticleEntity;

pub type DynArticlesDao = Arc<dyn ArticlesDaoTrait + Send + Sync>;

#[derive(Debug, Clone)]
pub struct CreatArticle {
    pub article: NewArticleValidated,
    pub author_id: Uuid,
    pub slug: String,
}

impl CreatArticle {
    pub fn new(article: NewArticleValidated, author_id: Uuid, slug: String) -> Self {
        Self {
            article,
            author_id,
            slug,
        }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
/// slugがコンフリクトした場合はNoneを返す
pub trait ArticlesDaoTrait {
    async fn create_article(
        &self,
        create_article: CreatArticle,
    ) -> Result<Option<ArticleEntity>, ConduitError>;
    async fn get_article_by_slug(&self, slug: &str) -> Result<Option<ArticleEntity>, ConduitError>;
    async fn update_article(
        &self,
        article_id: i32,
        slug: Option<String>,
        update_article: UpdateArticle,
    ) -> ConduitResult<ArticleEntity>;
}
