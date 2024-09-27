use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use realworld_axum_betashuttle::{
    dao::Daos,
    endpoints::{profiles::handler::ProfileRouter, users::handler::UserRouter},
    AppState,
};
use shuttle_runtime::SecretStore;

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
        .nest("/api", UserRouter::new_router(daos.clone()))
        .nest("/api", ProfileRouter::new_router(daos))
        .layer(Extension(state));

    Ok(router.into())
}
