use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{super::profiles::dto::Profile, entity::ArticleEntity};

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

impl NewArticle {
    pub fn into_validated(self) -> NewArticleValidated {
        NewArticleValidated {
            title: self.title.unwrap(),
            description: self.description.unwrap(),
            body: self.body.unwrap(),
            tag_list: self.tag_list.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct NewArticleValidated {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
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
impl Article {
    pub(crate) fn from_entity(
        article: ArticleEntity,
        profile: Profile,
        is_favorite_current_user: bool,
        favorites_count: i32,
    ) -> Article {
        Article {
            id: article.id,
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: vec![],
            created_at: article.created_at.to_string(),
            updated_at: article.updated_at.to_string(),
            favorited: is_favorite_current_user,
            favorites_count,
            author: profile,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateArticleRes {
    pub article: Article,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetArticleRes {
    pub article: Article,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateArticle {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub body: Option<String>,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateArticleReq {
    #[validate(nested)]
    pub article: UpdateArticle,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateArticleRes {
    pub article: Article,
}
