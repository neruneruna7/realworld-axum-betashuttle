use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Request},
    http::request::Parts,
    Extension, Json,
};
use serde::de::DeserializeOwned;
use uuid::Uuid;
use validator::Validate;

use crate::{error::ConduitError, services::jwt::JwtService, ArcState};

#[derive(Debug, Clone)]
pub struct ValidationExtractot<T>(pub T);

// #[async_trait]
// impl<S, T> FromRequestParts<S> for ValidationExtractot<T>
// where
//     T: DeserializeOwned + Validate,
//     S: Send + Sync,
// {
//     type Rejection = ConduitError;
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         parts.
//         // let t = Json::<T>::from
//         let Json(value) = Json::<T>::from_request_parts(parts, state).await?;
//         value.validate()?;
//         Ok(ValidationExtractot(value))
//     }
// }

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
pub struct RequiredAuth(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuth
where
    S: Send + Sync,
{
    type Rejection = ConduitError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        let token_value = headers
            .get("Authorization")
            .ok_or(ConduitError::Unauthorized)?;
        println!("header_value: {:?}", token_value);

        let Extension(state): Extension<ArcState> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                println!("error: {:?}", e);
                ConduitError::InternalServerError
            })?;
        let claim = JwtService::new(state).get_claim_from_token(token_value.to_str().unwrap())?;
        Ok(RequiredAuth(claim.user_id))
    }
}

// #[async_trait]
// impl<S> FromRequest<S> for RequiredAuth
// where
//     S: Send + Sync,
// {
//     type Rejection = ConduitError;

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let headers = req.headers().clone();
//         let token_value = headers
//             .get("Authorization")
//             .ok_or(ConduitError::Unauthorized)?;
//         println!("header_value: {:?}", token_value);

//         let Extension(state): Extension<ArcState> =
//             Extension::from_request(req, state).await.map_err(|e| {
//                 println!("error: {:?}", e);
//                 ConduitError::InternalServerError
//             })?;
//         let claim = JwtService::new(state).get_claim_from_token(token_value.to_str().unwrap())?;
//         Ok(RequiredAuth(claim.user_id))
//     }
// }
