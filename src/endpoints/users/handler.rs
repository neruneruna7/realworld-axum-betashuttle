use axum::{
    extract::State, http::StatusCode, routing::post, Json, Router,
};

use crate::{endpoints::users::dto::User, extractor::ValidationExtractot, AppState};

use super::dto::RegisterUserReq;

pub struct UserRouter;

impl UserRouter {
    pub fn new_router(state: AppState) -> Router {
        Router::new()
            .route("/users", post(Self::register_user))
            .with_state(state)
    }

    #[tracing::instrument(skip(state))]
    async fn register_user(
        state: State<AppState>,
        ValidationExtractot(req): ValidationExtractot<RegisterUserReq>,
    ) -> (StatusCode, Json<User>) {
        let req = req.user;
        let tmp_user = User {
            email: req.email.unwrap(),
            username: req.username.unwrap(),
            ..Default::default()
        };

        (StatusCode::OK, Json(tmp_user))
    }
}
