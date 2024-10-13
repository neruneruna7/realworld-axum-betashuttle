use std::sync::Arc;

use axum::async_trait;

use crate::error::ConduitResult;

use super::users::dao_trait::UsersDaoTrait;

#[async_trait]
pub trait DbClientTrait {
    async fn begin(&mut self) -> ConduitResult<()>; // トランザクション開始
    async fn commit(&mut self) -> ConduitResult<()>; // トランザクションコミット
    async fn rollback(&mut self) -> ConduitResult<()>; // トランザクションロールバック
}
