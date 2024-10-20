use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use realworld_axum_betashuttle::{
    core::{
        articles::dao_trait::DynArticlesDao, profiles::dao_trait::DynProfilesDao,
        tags::dao_trait::DynTagsDao, users::dao_trait::DynUsersDao,
    },
    dao::Daos,
    endpoints::{
        articles::ArticleRouter, favorites::FavoritesRouter, profiles::ProfileRouter,
        users::UserRouter,
    },
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

    let dyn_users_dao = Arc::new(daos.users) as DynUsersDao;
    let dyn_profiles_dao = Arc::new(daos.profiles) as DynProfilesDao;
    let dyn_articles_dao = Arc::new(daos.articles) as DynArticlesDao;
    let dyn_tags_dao = Arc::new(daos.tags) as DynTagsDao;
    let dyn_favorite_dao = Arc::new(daos.favorites);

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/api", UserRouter::new(dyn_users_dao.clone()).to_router())
        .nest(
            "/api",
            ProfileRouter::new(dyn_users_dao.clone(), dyn_profiles_dao.clone()).to_router(),
        )
        .nest(
            "/api",
            ArticleRouter::new(
                dyn_articles_dao.clone(),
                dyn_users_dao.clone(),
                dyn_tags_dao.clone(),
                dyn_favorite_dao.clone(),
            )
            .to_router(),
        )
        .nest(
            "/api",
            FavoritesRouter::new(
                dyn_profiles_dao.clone(),
                dyn_users_dao.clone(),
                dyn_articles_dao.clone(),
                dyn_favorite_dao.clone(),
            )
            .to_router(),
        )
        .layer(Extension(state));

    Ok(router.into())
}
