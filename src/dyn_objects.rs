use std::sync::Arc;

use sqlx::PgPool;

use crate::core::profiles::dao_trait::ProfilesDaoTrait;
// どのみち，コネクションの生成でPgPoolを使うので，ここで型エイリアスを指定しておく
pub type DynProfilesDao = Arc<dyn ProfilesDaoTrait<Connection = PgPool> + Send + Sync>;
