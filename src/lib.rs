use std::sync::Arc;

use sqlx::PgPool;

pub mod core;
pub mod dao;
pub mod dyn_objects;
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
