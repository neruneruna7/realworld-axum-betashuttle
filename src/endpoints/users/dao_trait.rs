use crate::{
    endpoints::users::{dto::PasswdHashedNewUser, entity::UserEntity},
    error::ConduitResult,
};
use axum::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UsersDaoTrait {
    async fn create_user(&self, new_user: PasswdHashedNewUser) -> ConduitResult<UserEntity>;
    async fn get_user_by_id(&self, user_id: Uuid) -> ConduitResult<UserEntity>;
    async fn get_user_by_email(&self, email: &str) -> ConduitResult<Option<UserEntity>>;
    async fn get_user_by_username(&self, username: &str) -> ConduitResult<Option<UserEntity>>;
    async fn update_user(&self, user: UserEntity) -> ConduitResult<UserEntity>;
}