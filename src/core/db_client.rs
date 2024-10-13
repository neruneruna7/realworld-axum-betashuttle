use std::sync::Arc;

use axum::async_trait;

use crate::error::ConduitResult;

pub type DynDbClient = Arc<dyn DbClientTrait + Send + Sync>;

#[async_trait]
pub trait DbClientTrait {
    async fn begin(&mut self) -> ConduitResult<()>; // トランザクション開始
    async fn commit(&mut self) -> ConduitResult<()>; // トランザクションコミット
    async fn rollback(&mut self) -> ConduitResult<()>; // トランザクションロールバック
}
