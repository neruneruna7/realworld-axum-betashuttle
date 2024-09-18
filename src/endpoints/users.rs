use axum::Router;
use validator::Validate;

mod dto;

pub fn router() -> Router {
    Router::new()
}
