use std::sync::Arc;

use axum::async_trait;

use crate::error::ConduitResult;

use super::entity::TagEntity;

pub type DynTagsDao = Arc<dyn TagDaoTrait + Send + Sync>;
#[async_trait]
pub trait TagDaoTrait {
    async fn create_tags(&self, tags: Vec<String>) -> ConduitResult<Vec<TagEntity>>;
    async fn get_tags_exists(&self, tags: Vec<String>) -> ConduitResult<Vec<TagEntity>>;
    async fn create_article_tags(&self, article_tag_ids: Vec<(i32, i32)>) -> ConduitResult<()>;
    async fn get_article_tags(&self, article_id: i32) -> ConduitResult<Vec<TagEntity>>;
}
