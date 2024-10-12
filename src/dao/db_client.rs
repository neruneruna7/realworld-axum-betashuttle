use core::error;

use anyhow::{anyhow, bail, Context as _};
use axum::async_trait;
use sqlx::{postgres::PgRow, query::Map, Executor, IntoArguments, PgPool, Postgres};

use crate::{
    core::db_client::DbClientTrait,
    error::{ConduitError, ConduitResult},
};

pub struct DbClient {
    pub pool: PgPool,
    pub tx: Option<sqlx::Transaction<'static, sqlx::Postgres>>,
}

impl DbClient {
    pub fn new(pool: PgPool) -> Self {
        Self { pool, tx: None }
    }

    /// クエリを実行する
    /// query_as!マクロを使って作成したクエリを実行する
    /// トランザクションが存在する場合はトランザクションを使う
    /// 実際実引数にとるのは，Mapから始まる型
    // ジェネリクス周りが非常に複雑
    // Tはクロージャで、引数にPgRowを取り、結果を返す
    // FはIntoArgumentsを実装している型
    // Oは返り値の型
    pub(crate) async fn execute_query<'q, F, A, O>(
        &mut self,
        query: Map<'q, Postgres, F, A>,
    ) -> ConduitResult<O>
    where
        // https://docs.rs/sqlx/latest/sqlx/query/struct.Map.html#impl-Execute%3C'q,+DB%3E-for-Map%3C'q,+DB,+F,+A%3E
        // を参考にジェネリクスのトレイト境界を設定
        F: FnMut(PgRow) -> Result<O, sqlx::Error> + Send + 'q,
        A: IntoArguments<'q, Postgres> + Send + 'q,
        O: Send + Unpin,
    {
        let result = if let Some(ref mut tx) = self.tx {
            // 型が&mut PgConnectionになるように調整する
            let t = &mut **tx;
            query.fetch_one(t).await
        } else {
            query.fetch_one(&self.pool).await
        }?;
        Ok(result)
    }

    // オプショナルなクエリを実行する
    pub(crate) async fn execute_query_optional<'q, F, A, O>(
        &mut self,
        query: Map<'q, Postgres, F, A>,
    ) -> ConduitResult<Option<O>>
    where
        F: FnMut(PgRow) -> Result<O, sqlx::Error> + Send + 'q,
        A: IntoArguments<'q, Postgres> + Send + 'q,
        O: Send + Unpin,
    {
        let result = if let Some(ref mut tx) = self.tx {
            let t = &mut **tx;
            query.fetch_optional(t).await
        } else {
            query.fetch_optional(&self.pool).await
        }?;
        Ok(result)
    }
}

#[async_trait]
impl DbClientTrait for DbClient {
    async fn begin(&mut self) -> ConduitResult<()> {
        if self.tx.is_some() {
            return Err(ConduitError::AnyhowError(anyhow!(
                "transaction already exists"
            )));
        }
        let txn = self.pool.begin().await?;
        self.tx = Some(txn);
        Ok(())
    }

    async fn commit(&mut self) -> ConduitResult<()> {
        if let Some(txn) = self.tx.take() {
            txn.commit().await?;
        }
        Ok(())
    }

    async fn rollback(&mut self) -> ConduitResult<()> {
        if let Some(txn) = self.tx.take() {
            txn.rollback().await?;
        }
        Ok(())
    }
}
