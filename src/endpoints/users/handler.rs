use axum::{
    http::StatusCode,
    routing::{get, post, put},
    Extension, Json, Router,
};

use tracing::info;
// ハンドラー周りでよくわからないエラーメッセージがでたら，
// #[debug_handler]をつけてデバッグするといい
#[allow(unused_imports)]
use axum_macros::debug_handler;

use crate::{
    core::users::{
        dao_trait::UsersDaoTrait,
        dto::{
            GetUserRes, LoginUserReq, LoginUserRes, RegisterUserReq, RegisterUserRes,
            UpdateUserReq, UpdateUserRes,
        },
    },
    error::{ConduitError, ConduitResult},
    extractor::{RequiredAuth, ValidationExtractor},
    services::{hash::PasswordHashService, jwt::JwtService},
    ArcState,
};

pub struct UserRouter<D> {
    users_dao: D,
}

impl<D> UserRouter<D>
where
    D: UsersDaoTrait + Send + Sync,
{
    pub fn new(dyn_users_dao: DynUsersDao) -> Self {
        Self {
            users_dao: dyn_users_dao,
        }
    }

    pub fn to_router(&self) -> Router {
        Router::new()
            .route("/users", post(Self::register_user))
            .route("/users/login", post(Self::login_user))
            .route("/user", get(Self::get_current_user))
            .route("/user", put(Self::update_user))
            .layer(Extension(self.users_dao.clone()))
    }

    // ログ出力結果にパスワードを含まないようにする
    // emailについては出力するようにする
    #[tracing::instrument(skip_all,fields(req_user = req.user.email))]
    pub async fn register_user(
        Extension(state): Extension<ArcState>,
        Extension(user_dao): Extension<DynUsersDao>,
        ValidationExtractor(req): ValidationExtractor<RegisterUserReq>,
    ) -> ConduitResult<(StatusCode, Json<RegisterUserRes>)> {
        let req = req.user;

        info!("creating password hash user: {:?}", &req.email);
        let hashed_user = PasswordHashService::hash_password_newuser(req)?;
        // ここにDBへの登録処理を書く

        info!(
            "password hashed successfully creating user: {:?}",
            &hashed_user.email
        );
        let user_entity = user_dao.create_user(hashed_user).await?;

        info!(
            "user created successfully generating token user {:?}",
            &user_entity.email
        );
        let token = JwtService::new(state.clone()).to_token(user_entity.id);
        let user = user_entity.into_dto_with_generated_token(token);
        let user_res = RegisterUserRes { user };

        Ok((StatusCode::OK, Json(user_res)))
    }

    #[tracing::instrument(skip(state, user_dao))]
    pub async fn get_current_user(
        Extension(state): Extension<ArcState>,
        Extension(user_dao): Extension<DynUsersDao>,
        RequiredAuth(user_id): RequiredAuth,
    ) -> ConduitResult<(StatusCode, Json<GetUserRes>)> {
        info!("retrieving user_id: {:?}", user_id);
        let user_entity = user_dao.get_user_by_id(user_id).await?;

        info!(
            "user retrieved successfully email{:?}, generating token",
            &user_entity.email
        );
        let token = JwtService::new(state.clone()).to_token(user_entity.id);

        let user = user_entity.into_dto_with_generated_token(token);
        let user_res = GetUserRes { user };

        Ok((StatusCode::OK, Json(user_res)))
    }

    #[tracing::instrument(skip_all,fields(req_user = req.user.email))]
    pub async fn login_user(
        Extension(state): Extension<ArcState>,
        Extension(user_dao): Extension<DynUsersDao>,
        ValidationExtractor(req): ValidationExtractor<LoginUserReq>,
    ) -> ConduitResult<(StatusCode, Json<LoginUserRes>)> {
        let req = req.user;

        let user_entity = user_dao.get_user_by_email(&req.email.unwrap()).await?;
        // let else文 Someを返してきたら束縛 そうでないならelseで書いた文を実行
        let Some(user_entity) = user_entity else {
            return Err(ConduitError::NotFound(String::from(
                "User not found: email is not exist",
            )));
        };
        info!(
            "user retrieved successfully, email:{:?}, verifying password",
            &user_entity.email
        );

        info!(
            "{:?}, {:?}",
            &user_entity.password.clone(),
            &req.password.clone().unwrap()
        );
        PasswordHashService::verify_password(&user_entity.password, &req.password.unwrap())
            .inspect_err(|e| {
                info!("invalid login, user: {:?}, err: {}", &user_entity.email, e);
            })?;

        info!("password verified successfully, generating token");
        let token = JwtService::new(state.clone()).to_token(user_entity.id);

        let user = user_entity.into_dto_with_generated_token(token);
        let user_res = LoginUserRes { user };

        Ok((StatusCode::OK, Json(user_res)))
    }

    // #[debug_handler]
    #[tracing::instrument(skip_all,fields(req_user = req.user.email))]
    pub async fn update_user(
        RequiredAuth(user_id): RequiredAuth,
        Extension(state): Extension<ArcState>,
        Extension(user_dao): Extension<DynUsersDao>,
        // Request本文を消費するエキストラクターは1つのみかつ引数の最後でなければならない
        // https://docs.rs/axum/0.7.6/axum/extract/index.html
        ValidationExtractor(req): ValidationExtractor<UpdateUserReq>,
    ) -> ConduitResult<(StatusCode, Json<UpdateUserRes>)> {
        let req = req.user;
        // Noneのフィールドを更新しないようにする
        // ユーザーをIDを使って取得
        // Noneのフィールドは取得したユーザーのフィールドで上書き
        // ユーザーを更新

        info!("retrieving user_id: {:?}", user_id);
        let user_entity = user_dao.get_user_by_id(user_id).await?;
        let user_entity = if req.password.is_some() {
            let user_entity = user_entity.update_user_entity(req);
            info!("hashing password for user: {:?}", &user_entity.email);
            PasswordHashService::hash_password_user(user_entity)?
        } else {
            user_entity.update_user_entity(req)
        };

        info!(
            "user retrieved successfully, email:{:?}, updating user",
            &user_entity.email
        );
        let user_entity = user_dao.update_user(user_entity).await?;

        info!(
            "user updated successfully, email:{:?}, generating token",
            &user_entity.email
        );
        let token = JwtService::new(state.clone()).to_token(user_entity.id);
        let user = user_entity.into_dto_with_generated_token(token);
        let user_res = UpdateUserRes { user };

        Ok((StatusCode::OK, Json(user_res)))
    }
}
