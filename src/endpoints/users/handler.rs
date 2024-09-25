use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use tracing::info;

use crate::{
    endpoints::users::dto::User,
    error::{ConduitError, ConduitResult, CustomArgon2Error},
    extractor::{RequiredAuth, ValidationExtractot},
    jwt::JwtService,
    ArcState,
};

use super::{
    dao::{PasswdHashedNewUser, UserDao},
    dto::{LoginUserReq, NewUser, RegisterUserReq},
};

pub struct UserRouter;

impl UserRouter {
    pub(crate) fn new_router() -> Router {
        Router::new()
            .route("/users", post(Self::register_user))
            .route("/users/login", post(Self::login_user))
            .route("/user", get(Self::get_current_user))
    }

    // ログ出力結果にパスワードを含まないようにする
    // emailについては出力するようにする
    #[tracing::instrument(skip_all,fields(req_user = req.user.email))]
    async fn register_user(
        Extension(state): Extension<ArcState>,
        ValidationExtractot(req): ValidationExtractot<RegisterUserReq>,
    ) -> ConduitResult<(StatusCode, Json<User>)> {
        let req = req.user;

        info!("creating password hash user: {:?}", &req.email);
        let hashed_user = hash_password_user(req)?;
        // ここにDBへの登録処理を書く

        info!(
            "password hashed successfully creating user: {:?}",
            &hashed_user.email
        );
        let user_dao = UserDao::new(state.pool.clone());
        let user_entity = user_dao.create_user(hashed_user).await?;

        info!(
            "user created successfully generating token user {:?}",
            &user_entity.email
        );
        let token = JwtService::new(state.clone()).to_token(user_entity.id);
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
        Extension(state): Extension<ArcState>,
        RequiredAuth(user_id): RequiredAuth,
    ) -> ConduitResult<(StatusCode, Json<User>)> {
        info!("retrieving user_id: {:?}", user_id);
        let dao = UserDao::new(state.pool.clone());
        let user_entity = dao.get_user_by_id(user_id).await?;

        info!(
            "user retrieved successfully email{:?}, generating token",
            &user_entity.email
        );
        let token = JwtService::new(state.clone()).to_token(user_entity.id);

        let user = User {
            email: user_entity.email,
            username: user_entity.username,
            bio: user_entity.bio,
            image: Some(user_entity.image),
            token,
        };

        Ok((StatusCode::OK, Json(user)))
    }

    #[tracing::instrument(skip_all,fields(req_user = req.user.email))]
    async fn login_user(
        Extension(state): Extension<ArcState>,
        ValidationExtractot(req): ValidationExtractot<LoginUserReq>,
    ) -> ConduitResult<(StatusCode, Json<User>)> {
        let req = req.user;

        let dao = UserDao::new(state.pool.clone());
        let user_entity = dao.get_user_by_email(&req.email.unwrap()).await?;
        // let else文 Someを返してきたら束縛 そうでないならelseで書いた文を実行
        let Some(user_entity) = user_entity else {
            return Err(ConduitError::NotFound(String::from(
                "User not found: email is not exist",
            )));
        };
        info!(
            "user retrieved successfully, email:{:?}, varifying password",
            &user_entity.email
        );

        varify_password(&user_entity.password, &req.password.unwrap()).map_err(|e| {
            info!("invalid login, user: {:?}", &user_entity.email);
            e
        })?;

        info!("password varified successfully, generating token");
        let token = JwtService::new(state.clone()).to_token(user_entity.id);

        let user = user_entity.into_dto_with_generated_token(token);

        Ok((StatusCode::OK, Json(user)))
    }
}

/// 成功した場合は何も返さない 失敗した場合はエラーを返す
fn varify_password(stored_pasword: &str, attemppt_password: &str) -> ConduitResult<()> {
    let expected = PasswordHash::new(stored_pasword).map_err(CustomArgon2Error)?;
    let argon2 = Argon2::default();
    argon2
        .verify_password(attemppt_password.as_bytes(), &expected)
        .map_err(CustomArgon2Error)?;
    Ok(())
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
