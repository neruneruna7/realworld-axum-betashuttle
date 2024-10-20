use serde::Serialize;

use crate::core::articles::dto::Article;

#[derive(Debug, Clone, Serialize)]
pub struct AddFavoriteRes {
    pub article: Article,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteFavoriteRes {
    pub article: Article,
}
