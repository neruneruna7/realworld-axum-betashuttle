use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tracing::info;

use crate::{
    endpoints::users::dto::User,
    error::{ConduitResult, CustomArgon2Error},
    extractor::{RequiredAuth, ValidationExtractot},
    jwt::JwtEncoder,
    AppState,
};

use super::{
    dao::{PasswdHashedNewUser, UserDao},
    dto::{NewUser, RegisterUserReq},
};

pub struct UserRouter;

impl UserRouter {
    pub(crate) fn new_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/users", post(Self::register_user))
            .route("/user", get(Self::get_current_user))
            .with_state(state)
    }

    #[tracing::instrument(skip(state))]
    async fn register_user(
        State(state): State<Arc<AppState>>,
        ValidationExtractot(req): ValidationExtractot<RegisterUserReq>,
    ) -> ConduitResult<(StatusCode, Json<User>)> {
        let req = req.user;

        let hashed_user = hash_password_user(req)?;
        // ここにDBへの登録処理を書く
        let user_dao = UserDao::new(state.pool.clone());
        let user_entity = user_dao.create_user(hashed_user).await?;
        let token = JwtEncoder::new(state.clone()).to_token(user_entity.id);
        let user = User {
            email: user_entity.email,
            username: user_entity.username,
            bio: user_entity.bio,
            image: Some(user_entity.image),
            token,
        };

        Ok((StatusCode::OK, Json(user)))
    }

    #[tracing::instrument(skip(state))]
    async fn get_current_user(
        State(state): State<Arc<AppState>>,
        RequiredAuth(user_id): RequiredAuth,
    ) -> ConduitResult<(StatusCode, Json<User>)> {
        info!("user_id: {:?}", user_id);
        // let req = req.user;

        // let hashed_user = hash_password_user(req)?;
        // // ここにDBへの登録処理を書く
        // let user_dao = UserDao::new(state.pool.clone());
        // let user_entity = user_dao.create_user(hashed_user).await?;
        // let token = JwtEncoder::new(user_entity.id).to_token(&state);
        // let user = User {
        //     email: user_entity.email,
        //     username: user_entity.username,
        //     bio: user_entity.bio,
        //     image: Some(user_entity.image),
        //     token,
        // };

        // Ok((StatusCode::OK, Json(user)))
        todo!()
    }
}

fn hash_password_user(req: NewUser) -> ConduitResult<PasswdHashedNewUser> {
    let hashed_pass = hash_password(&req.password.unwrap()).map(|password| {
        PasswdHashedNewUser::new(req.username.unwrap(), req.email.unwrap(), password)
    })?;
    Ok(hashed_pass)
}
fn hash_password(password: &str) -> ConduitResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    // OWASPチートシートにより決定
    // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
    // let argon2 = Argon2::new(
    //     Algorithm::Argon2id,
    //     argon2::Version::V0x13,
    //     Params::new(19000, 2, 1, None).unwrap(),
    // );
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(CustomArgon2Error)?;
    Ok(hash.to_string())
}
