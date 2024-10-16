use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct RegisterUserReq {
    #[validate(nested)]
    pub user: NewUser,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserRes {
    pub user: User,
}

#[derive(Debug, Clone, Validate, Deserialize, PartialEq)]
pub struct NewUser {
    #[validate(required)]
    pub username: Option<String>,
    #[validate(email, required)]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PasswdHashedNewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl PasswdHashedNewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
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

#[derive(Debug, Serialize)]
pub struct GetUserRes {
    pub user: User,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserReq {
    #[validate(nested)]
    pub user: UpdateUser,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserRes {
    pub user: User,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}
