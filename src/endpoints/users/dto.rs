use serde::Deserialize;
use validator::Validate;

#[derive(Debug,Validate, Deserialize)]
pub struct User {
    #[validate(email)]
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub imgae: Option<String>,
}

#[derive(Debug,Validate, Deserialize)]
pub struct NewUser {
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}
