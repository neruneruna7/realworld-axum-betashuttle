use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use axum_macros::debug_handler;
use slug::slugify;

use crate::{
    endpoints::articles::dto::NewArticleValidated,
    error::ConduitResult,
    extractor::{RequiredAuth, ValidationExtractot},
    ArcState,
};

use super::{
    dao_trait::DynArticlesDao,
    dto::{CreateArticleReq, CreateArticleRes},
};

pub struct ArticleRouter {
    article_dao: DynArticlesDao,
}

impl ArticleRouter {
    pub fn new(article_dao: DynArticlesDao) -> Self {
        Self { article_dao }
    }

    pub fn to_router(&self) -> Router {
        Router::new()
            .route("/articles", post(Self::create_article))
            .layer(Extension(self.article_dao.clone()))
    }

    #[tracing::instrument(skip_all)]
    // #[debug_handler]
    pub async fn create_article(
        RequiredAuth(user_id): RequiredAuth,
        Extension(article_dto): Extension<DynArticlesDao>,
        ValidationExtractot(req): ValidationExtractot<CreateArticleReq>,
    ) -> ConduitResult<(StatusCode, Json<CreateArticleRes>)> {
        let new_article = req.article;
        let new_article = new_article.into_validated();
        let slug = slugify(new_article.title.unwrap().as_str());

        let article = article_dto.create_article(new_article, user_id).await?;

        // Ok((StatusCode::CREATED, Json(CreateArticleRes { article })))
        todo!()
    }
}
