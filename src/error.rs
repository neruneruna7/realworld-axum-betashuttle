use std::fmt::Display;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::info;
use validator::ValidationErrors;

pub type ConduitResult<T> = Result<T, ConduitError>;

#[derive(Debug, Error)]
pub enum ConduitError {
    // DBの操作に失敗した場合
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    // パスワードのハッシュ化に失敗した場合
    #[error(transparent)]
    Argon2Error(#[from] CustomArgon2Error),
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
            // DBの操作に失敗した場合 サーバー側の問題
            Self::SqlxError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            // パスワードのハッシュ化に失敗した場合 サーバー側の問題
            Self::Argon2Error(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
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

#[derive(Debug)]
pub struct CustomArgon2Error(pub argon2::password_hash::Error);

impl Display for CustomArgon2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Argon2 error: {}", self.0)
    }
}

impl std::error::Error for CustomArgon2Error {}
