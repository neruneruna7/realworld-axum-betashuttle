use axum::Router;

pub struct UserRouter;

impl UserRouter {
    pub fn router() -> Router {
        Router::new()
    }

    async fn register_user() {}
}
