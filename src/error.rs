use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::info;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ConduitError {
    #[error(transparent)]
    ValidationErrpr(#[from] ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ConduitError {
    fn into_response(self) -> Response {
        info!("umimplemented error handling: {:?}", self);
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
