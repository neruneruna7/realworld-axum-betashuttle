use std::fmt::Display;

use axum::{
    extract::rejection::{ExtensionRejection, JsonRejection},
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
    // 未認証の場合
    #[error("Unauthorized")]
    Unauthorized,
    #[error("username or password is invalid")]
    InvalidLogin,
    #[error("{0}")]
    BadRequest(String),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("{0}")]
    NotFound(String),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    // DBの操作に失敗した場合
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    // パスワードのハッシュ化に失敗した場合
    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] CustomArgon2Error),
    // 現状，下記2つはPOSTでのリクエスト時に発生するエラー
    #[error(transparent)]
    // バリデーション失敗を意味する
    ValidationErrpr(#[from] ValidationErrors),
    #[error(transparent)]
    // Jsonへの変換が拒絶されたことを意味する
    AxumJsonRejection(#[from] JsonRejection),
    // Extensionの取得に失敗した場合
    #[error(transparent)]
    AxumExtensionRejection(#[from] ExtensionRejection),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("{0}")]
    Conflict(String),
}

impl IntoResponse for ConduitError {
    fn into_response(self) -> Response {
        info!("Error: {:?}", self);
        let (s, message) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
            // https://tex2e.github.io/rfc-translater/html/rfc9110.html
            // RFC 9110より 401: リクエストに認証資格情報が含まれている場合、401応答は、これらの資格情報に対して承認が拒否されたことを示します。
            Self::InvalidLogin => (StatusCode::UNAUTHORIZED, Self::InvalidLogin.to_string()),
            Self::NotFound(e) => (StatusCode::NOT_FOUND, e),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Self::InternalServerError.to_string(),
            ),
            Self::Conflict(e) => (StatusCode::CONFLICT, e),
            Self::BadRequest(e) => (StatusCode::BAD_REQUEST, e),
            Self::JwtError(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
            // DBの操作に失敗した場合 サーバー側の問題
            Self::SqlxError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            // パスワードのハッシュ化に失敗した場合 サーバー側の問題
            Self::Argon2Error(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            // 構文などは間違っていないが，データの制約に違反している場合
            Self::ValidationErrpr(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            // 与えられたJsonの構文やデータに不正があることを意味する？
            // ならば，クライアント側が悪いのでBAD_REQUESTを返すのが適切か 要検討
            Self::AxumJsonRejection(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            // Extensionの取得に失敗した場合 サーバー側の問題
            Self::AxumExtensionRejection(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::AnyhowError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        let body = Json(message);

        (s, body).into_response()
    }
}

#[derive(Debug)]
pub struct CustomArgon2Error(pub argon2::password_hash::Error);

impl Display for CustomArgon2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = self.0;
        write!(f, "Argon2 error: {}", e)
    }
}

impl std::error::Error for CustomArgon2Error {}
