use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Default)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserReq {
    #[validate(nested)]
    pub user: NewUser,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserRes {
    pub user: User,
}

#[derive(Debug, Validate, Deserialize)]
pub struct NewUser {
    #[validate(required)]
    pub username: Option<String>,
    #[validate(email, required)]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserReq {
    #[validate(nested)]
    pub user: LoginUser,
}

#[derive(Debug, Serialize)]
pub struct LoginUserRes {
    pub user: User,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email, required)]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserReq {
    #[validate(nested)]
    pub user: UpdateUser,
}

#[derive(Debug, Serialize)]
pub struct UserRes {
    pub user: User,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(email, required)]
    pub email: Option<String>,
    #[validate(required)]
    pub username: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}
