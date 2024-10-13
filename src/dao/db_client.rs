use anyhow::{anyhow, bail};
use axum::async_trait;
use sqlx::PgPool;

use crate::{
    core::db_client::DbClientTrait,
    error::{ConduitError, ConduitResult},
};

pub struct DbClient {
    pub pool: PgPool,
    pub txn: Option<sqlx::Transaction<'static, sqlx::Postgres>>,
}

#[async_trait]
impl DbClientTrait for DbClient {
    async fn begin(&mut self) -> ConduitResult<()> {
        if self.txn.is_some() {
            return Err(ConduitError::AnyhowError(anyhow!(
                "transaction already exists"
            )));
        }
        let txn = self.pool.begin().await?;
        self.txn = Some(txn);
        Ok(())
    }

    async fn commit(&mut self) -> ConduitResult<()> {
        if let Some(txn) = self.txn.take() {
            txn.commit().await?;
        }
        Ok(())
    }

    async fn rollback(&mut self) -> ConduitResult<()> {
        if let Some(txn) = self.txn.take() {
            txn.rollback().await?;
        }
        Ok(())
    }
}
