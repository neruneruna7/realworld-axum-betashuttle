use std::sync::Arc;

use axum::Extension;
use realworld_axum_betashuttle::{
    dao::users::UserDao,
    endpoints::users::{
        dto::{NewUser, RegisterUserReq, User},
        handler::UserRouter,
    },
    extractor::ValidationExtractot,
    AppState,
};
use sqlx::PgPool;

#[sqlx::test()]
async fn create_user(pool: PgPool) {
    // 1. Create a new user
    // 2. Check the response
    let state = AppState {
        pool,
        jwt_secret: "test".to_string(),
    };
    let user_dao = UserDao::new(state.pool.clone());

    let req = NewUser {
        email: Some("test@gmail.com".to_string()),
        password: Some("password".to_string()),
        username: Some("test".to_string()),
    };
    let req = RegisterUserReq { user: req };

    let create_user = UserRouter::register_user(
        Extension(Arc::new(state)),
        Extension(user_dao),
        ValidationExtractot(req.clone()),
    )
    .await
    .unwrap()
    .1
     .0;

    let req_user = User {
        email: req.user.email.unwrap(),
        username: req.user.username.unwrap(),
        bio: "".to_string(),
        image: Some("".to_string()),
        token: create_user.token.clone(),
    };

    assert_eq!(req_user, create_user);
}
