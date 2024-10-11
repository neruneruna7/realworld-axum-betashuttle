use std::sync::Arc;

use crate::endpoints::articles::dto::Article;
use crate::endpoints::articles::dto::NewArticle;
use crate::error::ConduitError;
use axum::async_trait;
use uuid::Uuid;

use super::dto::NewArticleValidated;
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
pub trait ArticlesDaoTrait {
    async fn create_article(
        &self,
        create_article: CreatArticle,
    ) -> Result<Option<ArticleEntity>, ConduitError>;
}
