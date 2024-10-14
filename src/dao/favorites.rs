use crate::{
    core::favorites::{dao_trait::FavoritesDaoTrait, entity::FavoritesEntity},
    error::ConduitResult,
};
use anyhow::Context as _;
use axum::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct FavoriteDao {
    pool: sqlx::PgPool,
}

impl FavoriteDao {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FavoritesDaoTrait for FavoriteDao {
    async fn add_favorite(&self, user_id: Uuid, article_id: i32) -> ConduitResult<FavoritesEntity> {
        // 論理削除フラグの更新を試みる
        // まだないなら新規作成
        // UPSERTすればいいだろう

        let favorite = sqlx::query_as!(
            FavoritesEntity,
            r#"
            INSERT INTO favorites (user_id, article_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, article_id) DO UPDATE
            SET is_deleted = false
            RETURNING *
            "#,
            user_id,
            article_id
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to add favorite")?;

        Ok(favorite)
    }

    async fn remove_favorite(
        &self,
        user_id: Uuid,
        article_id: i32,
    ) -> ConduitResult<FavoritesEntity> {
        // 論理削除フラグの更新
        let favorite = sqlx::query_as!(
            FavoritesEntity,
            r#"
            UPDATE favorites
            SET is_deleted = true
            WHERE user_id = $1 AND article_id = $2
            RETURNING *
            "#,
            user_id,
            article_id
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to remove favorite")?;

        Ok(favorite)
    }
}
