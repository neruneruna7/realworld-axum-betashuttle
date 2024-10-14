use std::sync::Arc;

use axum::async_trait;
use uuid::Uuid;

use crate::error::ConduitResult;

use super::entity::FavoritesEntity;

pub type DynFavoritesDao = Arc<dyn FavoritesDaoTrait + Send + Sync>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FavoritesDaoTrait {
    async fn add_favorite(&self, user_id: Uuid, article_id: i32) -> ConduitResult<FavoritesEntity>;
    async fn remove_favorite(
        &self,
        user_id: Uuid,
        article_id: i32,
    ) -> ConduitResult<FavoritesEntity>;
}
