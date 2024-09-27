use axum::{extract::Path, http::StatusCode, routing::post, Extension, Json, Router};
use axum_macros::debug_handler;
use tracing::info;

use crate::{
    dao::{profiles::ProfileDao, users::UserDao, Daos},
    endpoints::profiles::dto::Profile,
    error::{ConduitError, ConduitResult},
    extractor::RequiredAuth,
};

use super::dto::ProfileRes;

pub struct ProfileRouter;
impl ProfileRouter {
    pub(crate) fn new_router(daos: Daos) -> Router {
        Router::new()
            .route("/profiles/:username/follow", post(Self::follow_user))
            .layer(Extension(daos.users))
            .layer(Extension(daos.profiles))
    }

    #[tracing::instrument(skip(users, profiles))]
    // #[debug_handler]
    pub async fn follow_user(
        Path(username): Path<String>,
        Extension(users): Extension<UserDao>,
        Extension(profiles): Extension<ProfileDao>,
        RequiredAuth(current_user_id): RequiredAuth,
    ) -> ConduitResult<(StatusCode, Json<ProfileRes>)> {
        info!("recieved req: follow profile: {}", username);
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
