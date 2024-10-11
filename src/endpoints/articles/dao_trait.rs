use std::sync::Arc;

use crate::endpoints::articles::dto::Article;
use crate::endpoints::articles::dto::NewArticle;
use crate::error::ConduitError;
use axum::async_trait;
use uuid::Uuid;

use super::dto::NewArticleValidated;
use super::entity::ArticleEntity;

pub type DynArticlesDao = Arc<dyn ArticlesDaoTrait + Send + Sync>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ArticlesDaoTrait {
    async fn create_article(
        &self,
        article: NewArticleValidated,
        author_id: Uuid,
    ) -> Result<ArticleEntity, ConduitError>;
}
