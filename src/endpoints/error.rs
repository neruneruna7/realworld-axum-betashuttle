use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConduitError {}

impl IntoResponse for ConduitError {
    fn into_response(self) -> Response {
        unimplemented!()
    }
}
