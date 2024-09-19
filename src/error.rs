use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::info;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ConduitError {
    // 現状，下記2つはPOSTでのリクエスト時に発生するエラー
    #[error(transparent)]
    // バリデーション失敗を意味する
    ValidationErrpr(#[from] ValidationErrors),
    #[error(transparent)]
    // Jsonへの変換が拒絶されたことを意味する
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ConduitError {
    fn into_response(self) -> Response {
        info!("Error: {:?}", self);
        let (s, message) = match self {
            // 構文などは間違っていないが，データの制約に違反している場合
            Self::ValidationErrpr(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            // 与えられたJsonの構文やデータに不正があることを意味する？
            // ならば，クライアント側が悪いのでBAD_REQUESTを返すのが適切か 要検討
            Self::AxumJsonRejection(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };
        let body = Json(message);

        (s, body).into_response()
    }
}
