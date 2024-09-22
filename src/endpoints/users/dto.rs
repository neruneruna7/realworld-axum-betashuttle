use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Default)]
pub struct User {
    #[validate(email)]
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
#[derive(Debug, Validate, Deserialize)]
pub struct NewUser {
    #[validate(required)]
    pub username: Option<String>,
    #[validate(email, required)]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}
