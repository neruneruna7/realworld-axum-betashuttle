use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, routing::post, Extension, Json, Router};
use tracing::info;

use crate::{
    dao::Daos,
    endpoints::{profiles::dto::Profile, users::dao_trait::DynUsersDao},
    error::{ConduitError, ConduitResult},
    extractor::RequiredAuth,
};

use super::{
    dao_trait::{DynProfilesDao, ProfilesDaoTrait},
    dto::ProfileRes,
};

pub struct ProfileRouter;
impl ProfileRouter {
    pub fn new_router(daos: Daos) -> Router {
        // ここに書くのはなぁ，感はある
        let dyn_users_dao: DynUsersDao = Arc::new(daos.users);
        let dyn_profiles_dao: DynProfilesDao = Arc::new(daos.profiles);
        // うーん，どうせ1つしかないんだしdyn じゃなくて impl にしたいよなぁ
        Router::new()
            .route("/profiles/:username/follow", post(Self::follow_user))
            .layer(Extension(dyn_users_dao))
            .layer(Extension(dyn_profiles_dao))
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
}
