use anyhow::Context as _;
use axum::async_trait;
use sqlx::PgPool;

use crate::{
    core::articles::{
        dao_trait::{ArticlesDaoTrait, CreatArticle},
        entity::ArticleEntity,
    },
    error::ConduitError,
};

#[derive(Clone)]
pub struct ArticlesDao {
    pool: PgPool,
}

impl ArticlesDao {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticlesDaoTrait for ArticlesDao {
    async fn create_article(
        &self,
        create_article: CreatArticle,
    ) -> Result<Option<ArticleEntity>, ConduitError> {
        let article = sqlx::query_as!(
            ArticleEntity,
            r#"
            INSERT INTO articles (author_id, title, description, body, slug)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING
            RETURNING id, author_id, title, slug, description, body, created_at, updated_at
            "#,
            create_article.author_id,
            create_article.article.title,
            create_article.article.description,
            create_article.article.body,
            create_article.slug
        )
        .fetch_optional(&self.pool)
        .await
        .context("unexpected error: while inserting article")?;
        Ok(article)
    }

    async fn get_article_by_slug(&self, slug: &str) -> Result<Option<ArticleEntity>, ConduitError> {
        let article = sqlx::query_as!(
            ArticleEntity,
            r#"
            SELECT *
            FROM articles
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await
        .context("unexpected error: while fetching article")?;
        Ok(article)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{
            articles::{dao_trait::ArticlesDaoTrait, dto::NewArticleValidated},
            users::{dao_trait::UsersDaoTrait as _, dto::PasswdHashedNewUser},
        },
        dao::users::UserDao,
    };

    #[sqlx::test]
    async fn create_article(pool: PgPool) {
        // テスト用のユーザーを作成
        let user_dao = UserDao::new(pool.clone());
        // let new_user = NewUser {
        //     username: Some("username".to_string()),
        //     email: Some("email@example".to_string()),
        //     password: Some("password".to_string()),
        // };
        let new_user = PasswdHashedNewUser::new(
            "a".to_string(),
            "example@email.com".to_string(),
            "password".to_string(),
        );

        let user = user_dao
            .create_user(new_user)
            .await
            .expect("failed to create user");

        // テスト用の記事を作成
        let dao = ArticlesDao::new(pool);
        let create_article = CreatArticle::new(
            NewArticleValidated {
                title: "title".to_string(),
                description: "description".to_string(),
                body: "body".to_string(),
                tag_list: vec![],
            },
            user.id,
            "slug".to_string(),
        );
        let article = dao
            .create_article(create_article.clone())
            .await
            .expect("failed to create article");
        assert!(article.is_some());

        let article = dao
            .create_article(create_article)
            .await
            .expect("failed to create article");
        assert!(article.is_none());
    }
}
