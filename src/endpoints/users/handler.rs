use axum::{routing::post, Router};

pub struct UserRouter;

impl UserRouter {
    pub fn new_router() -> Router {
        Router::new().route("/users", post(Self::register_user))
    }

    async fn register_user() {
        unimplemented!()
    }
}
