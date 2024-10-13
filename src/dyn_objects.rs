use std::sync::Arc;

use sqlx::PgPool;

use crate::core::{
    articles::dao_trait::ArticlesDaoTrait, db_client::DbClientTrait,
    profiles::dao_trait::ProfilesDaoTrait, users::dao_trait::UsersDaoTrait,
};

pub type DynDbClient = Arc<dyn DbClientTrait + Send + Sync>;
// どのみち，コネクションの生成でPgPoolを使うので，ここで型エイリアスを指定しておく
pub type DynProfilesDao = Arc<dyn ProfilesDaoTrait<Connection = PgPool> + Send + Sync>;
pub type DynUsersDao = Arc<dyn UsersDaoTrait + Send + Sync>;
pub type DynArticlesDao = Arc<dyn ArticlesDaoTrait + Send + Sync>;
