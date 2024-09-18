use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
};
use thiserror::Error;
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
        unimplemented!()
    }
}
