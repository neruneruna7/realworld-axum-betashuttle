use axum::{routing::get, Router};
use endpoints::users::handler::UserRouter;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

pub mod endpoints;
pub mod error;
pub mod extractor;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] _secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> shuttle_axum::ShuttleAxum {
    let state = AppState { pool };

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/", UserRouter::new_router(state.clone()));

    Ok(router.into())
}
