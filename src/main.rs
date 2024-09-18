use axum::{routing::get, Router};
use endpoints::users::handler::UserRouter;
use shuttle_runtime::SecretStore;

pub mod endpoints;
pub mod error;
pub mod extractor;

#[derive(Clone)]
struct State {}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] _secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/api", UserRouter::new_router());

    Ok(router.into())
}
