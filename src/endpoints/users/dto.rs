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
