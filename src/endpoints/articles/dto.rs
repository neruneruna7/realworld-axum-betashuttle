use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::endpoints::profiles::dto::Profile;

#[derive(Debug, Clone, Validate, Deserialize, PartialEq)]
pub struct NewArticle {
    #[validate(required, length(min = 1))]
    pub title: Option<String>,
    #[validate(required)]
    pub description: Option<String>,
    #[validate(required, length(min = 1))]
    pub body: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateArticleReq {
    #[validate(nested)]
    pub article: NewArticle,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Article {
    #[serde(skip_serializing)]
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i32,
    pub author: Profile,
}

pub struct CreateArticleRes {
    pub article: Article,
}
