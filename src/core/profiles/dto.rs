use serde::Serialize;

use super::super::users::entity::UserEntity;
#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool,
}

impl Profile {
    pub fn from_user_entity(user: UserEntity, following: bool) -> Self {
        Self {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ProfileRes {
    pub profile: Profile,
}
