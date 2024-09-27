use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use dao::Daos;
use endpoints::users::handler::UserRouter;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

pub mod dao;
pub mod endpoints;
pub mod error;
pub mod extractor;
pub mod services;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    jwt_secret: String,
}

type ArcState = Arc<AppState>;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] _secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> shuttle_axum::ShuttleAxum {
    let state = AppState {
        pool,
        jwt_secret: _secrets.get("JWT_SECRET").unwrap(),
    };
    let state = Arc::new(state);
    let daos = Daos::new(state.pool.clone());

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/", UserRouter::new_router(daos))
        .layer(Extension(state));

    Ok(router.into())
}
