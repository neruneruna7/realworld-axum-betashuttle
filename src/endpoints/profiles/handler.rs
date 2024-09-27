use axum::Router;

use crate::dao::Daos;

pub struct UserRouter;
impl UserRouter {
    pub(crate) fn new_router(daos: Daos) -> Router {
        Router::new()
    }
}
