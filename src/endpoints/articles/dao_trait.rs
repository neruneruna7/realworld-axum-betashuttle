use std::sync::Arc;

use crate::endpoints::articles::dto::Article;
use crate::endpoints::articles::dto::NewArticle;
use crate::error::ConduitError;
use axum::async_trait;

pub type DynArticlesDao = Arc<dyn ArticlesDaoTrait + Send + Sync>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ArticlesDaoTrait {
    async fn create_article(&self, article: NewArticle) -> Result<Article, ConduitError>;
}
