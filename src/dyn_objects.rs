use std::sync::Arc;

use sqlx::PgPool;

use crate::core::profiles::dao_trait::ProfilesDaoTrait;

pub type DynProfilesDao = Arc<dyn ProfilesDaoTrait<Connection = PgPool> + Send + Sync>;
