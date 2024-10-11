use axum::{routing::post, Router};

use crate::error::ConduitResult;

pub struct ArticleRouter {}

impl ArticleRouter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn to_router(&self) -> Router {
        Router::new().route("/articles", post(Self::create_article))
    }
}
