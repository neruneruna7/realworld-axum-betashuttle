use serde::Serialize;

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct PrifileDto {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct ProfileRes {
    pub profile: PrifileDto,
}
