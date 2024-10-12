use std::sync::Arc;

use super::{dto::PasswdHashedNewUser, entity::UserEntity};
use crate::error::ConduitResult;
use axum::async_trait;
use uuid::Uuid;

// pub type DynUsersDao = Arc<dyn UsersDaoTrait + Send + Sync>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UsersDaoTrait {
    // トランザクションを使うために，&mut selfを引数に取る
    async fn create_user(&mut self, new_user: PasswdHashedNewUser) -> ConduitResult<UserEntity>;
    async fn get_user_by_id(&mut self, user_id: Uuid) -> ConduitResult<UserEntity>;
    async fn get_user_by_email(&mut self, email: &str) -> ConduitResult<Option<UserEntity>>;
    async fn get_user_by_username(&mut self, username: &str) -> ConduitResult<Option<UserEntity>>;
    async fn update_user(&mut self, user: UserEntity) -> ConduitResult<UserEntity>;
}
