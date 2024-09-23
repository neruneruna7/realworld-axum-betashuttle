use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRef, FromRequest, Request, State},
    Extension, Json,
};
use serde::de::DeserializeOwned;
use tracing::info;
use validator::Validate;

use crate::{error::ConduitError, jwt::JwtService, AppState, ArcState};

#[derive(Debug, Clone)]
pub struct ValidationExtractot<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidationExtractot<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    // ひとまず確認のためのRejection型を定義
    // 後でカスタムエラーを定義する
    type Rejection = ConduitError;

    #[doc = " Perform the extraction."]
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidationExtractot(value))
    }
}

/// Authorization token headerからJWTを抽出する
pub struct RequiredAuth(pub String);

#[async_trait]
impl<S> FromRequest<S> for RequiredAuth
where
    S: Send + Sync,
{
    type Rejection = ConduitError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers().clone();
        let token_value = headers
            .get("Authorization")
            .ok_or(ConduitError::Unauthorized)?;
        println!("header_value: {:?}", token_value);

        let Extension(state): Extension<ArcState> =
            Extension::from_request(req, state).await.map_err(|e| {
                println!("error: {:?}", e);
                ConduitError::InternalServerError
            })?;
        let claim = JwtService::new(state).get_claim_from_token(token_value.to_str().unwrap())?;
        Ok(RequiredAuth(claim.user_id.to_string()))
    }
}
