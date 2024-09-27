use std::sync::Arc;

use sqlx::PgPool;

pub mod dao;
pub mod endpoints;
pub mod error;
pub mod extractor;
pub mod services;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

type ArcState = Arc<AppState>;
