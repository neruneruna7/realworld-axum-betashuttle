use std::sync::Arc;

use crate::error::ConduitError;
use axum::async_trait;
use uuid::Uuid;

use super::dto::NewArticleValidated;
use super::entity::ArticleEntity;

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
// #[cfg_attr(test, mockall::automock)]
#[async_trait]
/// slugがコンフリクトした場合はNoneを返す
pub trait ArticlesDaoTrait {
    // トランザクションを使えるようにするため，引数としてコネクションを取る
    // コネクションの型は実装時に決定し，かつ1つに定まるため，関連型として定義
    // type Connection;
    async fn create_article(
        &self,
        // conn: Self::Connection,
        create_article: CreatArticle,
    ) -> Result<Option<ArticleEntity>, ConduitError>;
}
