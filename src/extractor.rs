use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    BoxError, Error, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

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
    type Rejection = (StatusCode, String);

    #[doc = " Perform the extraction."]
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await.unwrap();
        // 後でカスタムエラーを定義し，?を使ってエラーを返す
        value.validate().unwrap();
        Ok(ValidationExtractot(value))
    }
}
