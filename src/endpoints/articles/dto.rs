use serde::Deserialize;
use validator::Validate;

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
