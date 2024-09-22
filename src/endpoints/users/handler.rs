use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{
    endpoints::users::dto::User,
    error::{ConduitResult, CustomArgon2Error},
    extractor::ValidationExtractot,
    AppState,
};

use super::{
    dao::{PasswdHashedNewUser, UserDao},
    dto::{NewUser, RegisterUserReq},
};

pub struct UserRouter;

impl UserRouter {
    pub(crate) fn new_router(state: AppState) -> Router {
        Router::new()
            .route("/users", post(Self::register_user))
            .with_state(state)
    }

    #[tracing::instrument(skip(state))]
    async fn register_user(
        state: State<AppState>,
        ValidationExtractot(req): ValidationExtractot<RegisterUserReq>,
    ) -> ConduitResult<(StatusCode, Json<User>)> {
        let req = req.user;
        let tmp_user = User {
            email: req.email.clone().unwrap(),
            username: req.username.clone().unwrap(),
            ..Default::default()
        };

        let hashed_user = hash_password_user(req)?;
        // ここにDBへの登録処理を書く
        let user_dao = UserDao::new(state.pool.clone());
        let user = user_dao.create_user(hashed_user).await?;

        Ok((StatusCode::OK, Json(tmp_user)))
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
        .map_err(|e| CustomArgon2Error(e))?;
    Ok(hash.to_string())
}
