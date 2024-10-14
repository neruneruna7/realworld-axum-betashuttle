use anyhow::Context as _;
use axum::async_trait;
use sqlx::PgPool;

use crate::{
    core::articles::{
        dao_trait::{ArticlesDaoTrait, CreatArticle},
        dto::UpdateArticle,
        entity::ArticleEntity,
    },
    error::{ConduitError, ConduitResult},
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

    async fn update_article(
        &self,
        article_id: i32,
        slug: Option<String>,
        update_article: UpdateArticle,
    ) -> ConduitResult<ArticleEntity> {
        // Noneのところは更新しない
        // titleの更新に伴って，slugも更新する
        // が，slugが衝突したら更新しない
        let article = sqlx::query_as!(
            ArticleEntity,
            r#"
            UPDATE articles
            SET title = COALESCE($2, title),
                description = COALESCE($3, description),
                body = COALESCE($4, body),
                slug = COALESCE($5, slug)
            WHERE id = $1
            RETURNING *
            "#,
            article_id,
            update_article.title,
            update_article.description,
            update_article.body,
            slug,
        )
        .fetch_one(&self.pool)
        .await
        .context("unexpected error: while updating article")?;
        Ok(article)
    }

    // スラグをもとに記事削除 削除した記事を返す
    async fn delete_article_by_slug(&self, slug: &str) -> ConduitResult<ArticleEntity> {
        let article = sqlx::query_as!(
            ArticleEntity,
            r#"
            DELETE FROM articles
            WHERE slug = $1
            RETURNING *
            "#,
            slug
        )
        .fetch_one(&self.pool)
        .await
        .context("unexpected error: while deleting article")?;
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

    #[sqlx::test]
    async fn get_article_by_slug(pool: PgPool) {
        // テスト用のユーザーを作成
        let user_dao = UserDao::new(pool.clone());
        let new_user =
            PasswdHashedNewUser::new("a".to_string(), "email".to_string(), "password".to_string());
        let user = user_dao
            .create_user(new_user)
            .await
            .expect("failed to create user");

        // テスト用の記事を作成
        let dao = ArticlesDao::new(pool.clone());
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

        let created_article = dao
            .create_article(create_article)
            .await
            .expect("failed to create article");

        let article = dao
            .get_article_by_slug("slug")
            .await
            .expect("failed to get article");

        assert_eq!(article.unwrap(), created_article.unwrap());
    }

    #[sqlx::test]
    async fn update_article(pool: PgPool) {
        // テスト用のユーザーを作成
        let user_dao = UserDao::new(pool.clone());
        let new_user =
            PasswdHashedNewUser::new("a".to_string(), "email".to_string(), "password".to_string());
        let user = user_dao
            .create_user(new_user)
            .await
            .expect("failed to create user");

        // テスト用の記事を作成
        let dao = ArticlesDao::new(pool.clone());
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

        let created_article = dao
            .create_article(create_article)
            .await
            .expect("failed to create article")
            .unwrap();

        let updated_article = dao
            .update_article(
                created_article.id,
                Some("slug".to_string()),
                UpdateArticle {
                    title: Some("new title".to_string()),
                    description: Some("new description".to_string()),
                    body: None,
                },
            )
            .await
            .expect("failed to update article");

        assert_eq!(updated_article.title, "new title");
        assert_eq!(updated_article.description, "new description");
        assert_eq!(updated_article.body, "body");
        assert_eq!(updated_article.slug, "slug");
    }

    // スラグコンフリクト時に更新しないことを確認
    #[sqlx::test]
    #[should_panic]
    async fn update_article_conflict(pool: PgPool) {
        // テスト用のユーザーを作成
        let user_dao = UserDao::new(pool.clone());
        let new_user =
            PasswdHashedNewUser::new("a".to_string(), "email".to_string(), "password".to_string());
        let user = user_dao
            .create_user(new_user)
            .await
            .expect("failed to create user");

        // テスト用の記事1を作成
        let dao = ArticlesDao::new(pool.clone());
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

        let _created_article = dao
            .create_article(create_article)
            .await
            .expect("failed to create article")
            .unwrap();

        // テスト用の記事2を作成
        let create_article = CreatArticle::new(
            NewArticleValidated {
                title: "title".to_string(),
                description: "description".to_string(),
                body: "body".to_string(),
                tag_list: vec![],
            },
            user.id,
            "slug2".to_string(),
        );

        let created_article2 = dao
            .create_article(create_article)
            .await
            .expect("failed to create article")
            .unwrap();

        // 記事2のスラグを記事1のスラグと同じものに更新
        let _updated_article = dao
            .update_article(
                created_article2.id,
                Some("slug".to_string()),
                UpdateArticle {
                    title: Some("new title".to_string()),
                    description: Some("new description".to_string()),
                    body: None,
                },
            )
            .await
            .expect("failed to update article");
    }
}
