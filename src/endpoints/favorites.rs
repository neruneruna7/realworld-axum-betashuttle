use axum::{extract::Path, http::StatusCode, routing::post, Extension, Json, Router};
use tracing::info;

use crate::{
    core::{
        articles::{dao_trait::DynArticlesDao, dto::Article},
        favorites::{dao_trait::DynFavoritesDao, dto::AddFavoriteRes},
        profiles::{dao_trait::DynProfilesDao, dto::Profile},
        users::dao_trait::DynUsersDao,
    },
    error::{ConduitError, ConduitResult},
    extractor::RequiredAuth,
};

pub struct FavoritesRouter {
    dyn_profiles_dao: DynProfilesDao,
    dyn_users_dao: DynUsersDao,
    dyn_articles_dao: DynArticlesDao,
    dyn_favorite_dao: DynFavoritesDao,
}

impl FavoritesRouter {
    pub fn new(
        dyn_profiles_dao: DynProfilesDao,
        dyn_users_dao: DynUsersDao,
        dyn_articles_dao: DynArticlesDao,
        dyn_favorite_dao: DynFavoritesDao,
    ) -> Self {
        Self {
            dyn_profiles_dao,
            dyn_users_dao,
            dyn_articles_dao,
            dyn_favorite_dao,
        }
    }

    pub fn to_router(&self) -> Router {
        Router::new()
            .route(
                "/articles/:slug/favorite",
                post(Self::add_favorite_article).delete(Self::delete_favorite_article),
            )
            .layer(Extension(self.dyn_profiles_dao.clone()))
            .layer(Extension(self.dyn_users_dao.clone()))
            .layer(Extension(self.dyn_articles_dao.clone()))
            .layer(Extension(self.dyn_favorite_dao.clone()))
    }

    // いいねエンドポイント
    #[tracing::instrument(skip(article_dao, favorite_dao, user_dao, profile_dao))]
    pub async fn add_favorite_article(
        Path(slug): Path<String>,
        RequiredAuth(current_user_id): RequiredAuth,
        Extension(profile_dao): Extension<DynProfilesDao>,
        Extension(user_dao): Extension<DynUsersDao>,
        Extension(article_dao): Extension<DynArticlesDao>,
        Extension(favorite_dao): Extension<DynFavoritesDao>,
    ) -> ConduitResult<(StatusCode, Json<AddFavoriteRes>)> {
        info!("add favorite article");
        // いいねする記事を取得
        let article = article_dao.get_article_by_slug(&slug).await?;
        let Some(article) = article else {
            info!("article not found");
            return Err(ConduitError::NotFound("article not found".to_string()));
        };

        // いいねを追加
        favorite_dao
            .add_favorite(current_user_id, article.id)
            .await?;

        // 返すデータを作成
        // 記事のいいね数を取得
        let favorites = favorite_dao.get_favorites_by_article_id(article.id).await?;
        // current_userが記事をいいねしているかどうかを探す
        let is_favorite_current_user = favorites
            .iter()
            .any(|favorite| favorite.user_id == current_user_id);

        info!("favorite added");
        // 記事を書いたユーザーを取得
        let author = user_dao.get_user_by_id(article.author_id).await?;
        // フォローしているかどうかを取得
        let following = profile_dao.is_follow(current_user_id, author.id).await?;
        let following = if let Some(_u) = following {
            true
        } else {
            false
        };
        // Profileを作成
        let profile = Profile::from_user_entity(author, following);
        // Articleを作成
        let article = Article::from_entity(
            article,
            profile,
            is_favorite_current_user,
            favorites.len() as i32,
        );

        Ok((StatusCode::OK, Json(AddFavoriteRes { article })))
    }

    // いいね解除エンドポイント
    #[tracing::instrument(skip(article_dao, favorite_dao, user_dao, profile_dao))]
    pub async fn delete_favorite_article(
        Path(slug): Path<String>,
        RequiredAuth(current_user_id): RequiredAuth,
        Extension(profile_dao): Extension<DynProfilesDao>,
        Extension(user_dao): Extension<DynUsersDao>,
        Extension(article_dao): Extension<DynArticlesDao>,
        Extension(favorite_dao): Extension<DynFavoritesDao>,
    ) -> ConduitResult<(StatusCode, Json<AddFavoriteRes>)> {
        info!("delete favorite article");
        // いいね削除する記事を取得
        let article = article_dao.get_article_by_slug(&slug).await?;
        let Some(article) = article else {
            info!("article not found");
            return Err(ConduitError::NotFound("article not found".to_string()));
        };

        // いいねを削除
        favorite_dao
            .remove_favorite(current_user_id, article.id)
            .await?;

        // 返すデータを作成
        // 記事のいいね数を取得
        let favorites = favorite_dao.get_favorites_by_article_id(article.id).await?;
        // current_userが記事をいいねしているかどうかを探す
        let is_favorite_current_user = favorites
            .iter()
            .any(|favorite| favorite.user_id == current_user_id);

        info!("favorite deleted");
        // 記事を書いたユーザーを取得
        let author = user_dao.get_user_by_id(article.author_id).await?;
        // フォローしているかどうかを取得
        let following = profile_dao.is_follow(current_user_id, author.id).await?;
        let following = if let Some(_u) = following {
            true
        } else {
            false
        };
        // Profileを作成
        let profile = Profile::from_user_entity(author, following);
        // Articleを作成
        let article = Article::from_entity(
            article,
            profile,
            is_favorite_current_user,
            favorites.len() as i32,
        );

        Ok((StatusCode::OK, Json(AddFavoriteRes { article })))
    }
}
