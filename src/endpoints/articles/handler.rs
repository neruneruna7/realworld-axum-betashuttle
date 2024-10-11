use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use axum_macros::debug_handler;

use crate::{
    error::ConduitResult,
    extractor::{RequiredAuth, ValidationExtractot},
    ArcState,
};

use super::dto::{CreateArticleReq, CreateArticleRes};

pub struct ArticleRouter {}

impl ArticleRouter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn to_router(&self) -> Router {
        Router::new().route("/articles", post(Self::create_article))
    }

    #[tracing::instrument(skip_all)]
    // #[debug_handler]
    pub async fn create_article(
        RequiredAuth(user_id): RequiredAuth,
        Extension(state): Extension<ArcState>,
        ValidationExtractot(req): ValidationExtractot<CreateArticleReq>,
    ) -> ConduitResult<(StatusCode, Json<CreateArticleRes>)> {
        todo!()
    }
}
