use axum::async_trait;
use sqlx::PgPool;

use crate::{
    core::tags::{dao_trait::TagDaoTrait, entiry::TagEntity},
    error::ConduitResult,
};

#[derive(Clone)]
pub struct TagsDao {
    pool: PgPool,
}

impl TagsDao {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagDaoTrait for TagsDao {
    async fn create_tags(&self, tags: Vec<String>) -> ConduitResult<Vec<TagEntity>> {
        // バルクインサートを行う
        // unnest関数を使って、配列をテーブルに展開する
        let tags_entity = sqlx::query_as!(
            TagEntity,
            r#"
            INSERT INTO tags (tag)
            SELECT unnest($1::text[])
            ON CONFLICT (tag) DO NOTHING
            RETURNING *
            "#,
            &tags
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags_entity)
    }

    async fn get_tags_exists(&self, tags: Vec<String>) -> ConduitResult<Vec<TagEntity>> {
        // 引数で渡されたタグ名のうち，存在するタグだけを返す
        let tags_entity = sqlx::query_as!(
            TagEntity,
            r#"
            SELECT * FROM tags
            WHERE tag = ANY($1)
            "#,
            &tags
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags_entity)
    }

    async fn create_article_tags(&self, article_tag_ids: Vec<(i32, i32)>) -> ConduitResult<()> {
        // unzip
        let (article_ids, tag_ids): (Vec<i32>, Vec<i32>) = article_tag_ids.iter().cloned().unzip();
        // バルクインサートを行う
        // unnest関数を使って、配列をテーブルに展開する
        sqlx::query!(
            r#"
            INSERT INTO article_tags (article_id, tag_id)
            SELECT * FROM unnest($1::int[], $2::int[])
            ON CONFLICT DO NOTHING
            "#,
            &article_ids,
            &tag_ids
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_article_tags(&self, article_id: i32) -> ConduitResult<Vec<TagEntity>> {
        let tags_entity = sqlx::query_as!(
            TagEntity,
            r#"
            SELECT tags.* FROM tags
            JOIN article_tags ON tags.id = article_tags.tag_id
            WHERE article_tags.article_id = $1
            "#,
            article_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags_entity)
    }
}

// test
#[cfg(test)]
mod tests {
    use crate::{
        core::{
            articles::{
                dao_trait::{ArticlesDaoTrait as _, CreatArticle},
                dto::NewArticleValidated,
            },
            users::dao_trait::UsersDaoTrait as _,
        },
        dao::articles::ArticlesDao,
    };

    use super::*;

    #[sqlx::test]
    async fn test_create_tags(pool: PgPool) {
        let dao = TagsDao::new(pool);

        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let tags_entity = dao.create_tags(tags).await.unwrap();

        assert_eq!(tags_entity.len(), 2);
        assert_eq!(tags_entity[0].tag, "tag1");
        assert_eq!(tags_entity[1].tag, "tag2");
    }

    #[sqlx::test]
    async fn test_get_tags_exists(pool: PgPool) {
        let dao = TagsDao::new(pool);

        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let _ = dao.create_tags(tags).await.unwrap();

        let tags = vec![
            "tag1".to_string(),
            "tag2".to_string(),
            "tag3_no_Exsitsts".to_string(),
        ];
        let tags_entity = dao.get_tags_exists(tags).await.unwrap();

        assert_eq!(tags_entity.len(), 2);
        assert_eq!(tags_entity[0].tag, "tag1");
        assert_eq!(tags_entity[1].tag, "tag2");
        assert_eq!(tags_entity.get(2), None);
    }

    #[sqlx::test]
    async fn test_create_article_tags(pool: PgPool) {
        // テストユーザーを作成
        let users_dao = crate::dao::users::UserDao::new(pool.clone());
        let new_user = crate::core::users::dto::PasswdHashedNewUser {
            username: "username".to_string(),
            email: "email".to_string(),
            password: "password".to_string(),
        };
        let user = users_dao.create_user(new_user).await.unwrap();

        // テスト記事を作成
        let articles_dao = ArticlesDao::new(pool.clone());
        let new_article_validated = NewArticleValidated {
            title: "title".to_string(),
            description: "description".to_string(),
            body: "body".to_string(),
            tag_list: vec!["tag1".to_string(), "tag2".to_string()],
        };
        let tags = new_article_validated.tag_list.clone();

        let craate_article = CreatArticle {
            article: new_article_validated,
            author_id: user.id,
            slug: "title".to_string(),
        };

        let article = articles_dao
            .create_article(craate_article)
            .await
            .unwrap()
            .unwrap();

        let dao = TagsDao::new(pool);

        let tags_entity = dao.create_tags(tags).await.unwrap();

        let article_tag_ids = vec![
            (article.id, tags_entity[0].id),
            (article.id, tags_entity[1].id),
        ];
        dao.create_article_tags(article_tag_ids).await.unwrap();
    }
}
