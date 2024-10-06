use std::{collections::HashMap, num::ParseIntError};

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use tracing::info;

use crate::{
    endpoints::{profiles::dto::Profile, users::dao_trait::DynUsersDao},
    error::{ConduitError, ConduitResult},
    extractor::{OptionalAuth, RequiredAuth},
};

use super::{dao_trait::DynProfilesDao, dto::ProfileRes};

pub struct ProfileRouter {
    dyn_users_dao: DynUsersDao,
    dyn_profiles_dao: DynProfilesDao,
}
impl ProfileRouter {
    pub fn new(dyn_users_dao: DynUsersDao, dyn_profiles_dao: DynProfilesDao) -> Self {
        Self {
            dyn_users_dao,
            dyn_profiles_dao,
        }
    }

    pub fn to_router(&self) -> Router {
        Router::new()
            .route("/profiles/:username/follow", post(Self::follow_user))
            .route("/profiles/:username", get(Self::get_profile_by_username))
            .layer(Extension(self.dyn_users_dao.clone()))
            .layer(Extension(self.dyn_profiles_dao.clone()))
    }

    #[tracing::instrument(skip(users, profiles))]
    // #[debug_handler]
    pub async fn follow_user(
        Path(username): Path<String>,
        Extension(users): Extension<DynUsersDao>,
        Extension(profiles): Extension<DynProfilesDao>,
        RequiredAuth(current_user_id): RequiredAuth,
    ) -> ConduitResult<(StatusCode, Json<ProfileRes>)> {
        info!("received req: follow profile: {}", username);
        let followed_user = users.get_user_by_username(&username).await?;
        let Some(followed_user) = followed_user else {
            return Err(ConduitError::NotFound("user not found".to_string()));
        };

        let is_following = profiles
            .get_user_following(followed_user.id)
            .await?
            .iter()
            .any(|f| f.follower_id == current_user_id);

        if !is_following {
            let _ = profiles
                .following_user(current_user_id, followed_user.id)
                .await?;
        }

        info!(
            "following: from user_id: {}, to user_id: {}",
            current_user_id, followed_user.id
        );
        let profile = Profile::from_user_entity(followed_user, true);
        let profile_res = ProfileRes { profile };

        Ok((StatusCode::OK, Json(profile_res)))
    }

    #[tracing::instrument(skip(users, profiles))]
    pub async fn get_profile_by_username(
        Path(params): Path<HashMap<String, String>>,
        Extension(users): Extension<DynUsersDao>,
        Extension(profiles): Extension<DynProfilesDao>,
        OptionalAuth(current_user_id): OptionalAuth,
    ) -> ConduitResult<(StatusCode, Json<ProfileRes>)> {
        let user_name = params
            .get("username")
            .ok_or(ConduitError::NotFound(String::from("invalid param")))?;
        info!("received req: get profile by username: {}", user_name);

        // usernameからユーザー情報を取得
        let user_entity = users.get_user_by_username(user_name).await?;
        let Some(user_entity) = user_entity else {
            return Err(ConduitError::NotFound("profile not found".to_string()));
        };

        // current_user_idがある場合，フォローしているかどうかを取得
        let is_following = if let Some(user_id) = current_user_id {
            let is_following = profiles
                .get_user_following(user_entity.id)
                .await?
                .iter()
                .any(|f| f.follower_id == user_id);
            is_following
        } else {
            false
        };

        let profile = Profile::from_user_entity(user_entity, is_following);
        let profile_res = ProfileRes { profile };

        Ok((StatusCode::OK, Json(profile_res)))
    }
}
