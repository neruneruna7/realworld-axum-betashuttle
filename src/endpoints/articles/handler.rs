use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use slug::slugify;
use tracing::info;

use crate::{
    core::{
        articles::{
            dao_trait::{CreatArticle, DynArticlesDao},
            dto::{Article, CreateArticleReq, CreateArticleRes},
        },
        profiles::dto::Profile,
        tags::dao_trait::DynTagsDao,
        users::dao_trait::DynUsersDao,
    },
    error::{ConduitError, ConduitResult},
    extractor::{RequiredAuth, ValidationExtractor},
};

pub struct ArticleRouter {
    article_dao: DynArticlesDao,
    user_dao: DynUsersDao,
    tag_dao: DynTagsDao,
}

impl ArticleRouter {
    pub fn new(article_dao: DynArticlesDao, user_dao: DynUsersDao, tag_dao: DynTagsDao) -> Self {
        Self {
            article_dao,
            user_dao,
            tag_dao,
        }
    }

    pub fn to_router(&self) -> Router {
        Router::new()
            .route("/articles", post(Self::create_article))
            .layer(Extension(self.article_dao.clone()))
            .layer(Extension(self.user_dao.clone()))
            .layer(Extension(self.tag_dao.clone()))
    }

    #[tracing::instrument(skip_all)]
    // #[debug_handler]
    pub async fn create_article(
        RequiredAuth(user_id): RequiredAuth,
        Extension(user_dao): Extension<DynUsersDao>,
        Extension(article_dao): Extension<DynArticlesDao>,
        Extension(tag_dao): Extension<DynTagsDao>,
        ValidationExtractor(req): ValidationExtractor<CreateArticleReq>,
    ) -> ConduitResult<(StatusCode, Json<CreateArticleRes>)> {
        info!("create_article");
        // バリデーション済みなのでそのことを示す
        let new_article = req.article.into_validated();

        // タグを作成
        let tags = tag_dao.create_tags(new_article.tag_list.clone()).await?;
        info!("new tag created: {:?}", tags);
        // タグIDを取得
        let tags = tag_dao
            .get_tags_exists(new_article.tag_list.clone())
            .await?;

        // スラグをタイトルから生成
        let slug = slugify(new_article.title.as_str());

        // 記事を作成
        let create_article = CreatArticle::new(new_article, user_id, slug);
        let article = article_dao.create_article(create_article).await?;

        // スラグはユニークである制約があるため，Noneの場合はエラー
        let Some(article) = article else {
            return Err(ConduitError::Conflict("slug already exists".to_string()));
        };

        // 記事とタグの関連付け
        let article_tag_ids = tags
            .iter()
            .map(|tag| (article.id, tag.id))
            .collect::<Vec<(i32, i32)>>();
        tag_dao.create_article_tags(article_tag_ids).await?;

        // 記事の作者(自分)を取得
        let user_entity = user_dao.get_user_by_id(user_id).await?;
        let author = Profile {
            username: user_entity.username,
            bio: user_entity.bio,
            image: user_entity.image,
            following: false,
        };

        let article = Article {
            id: article.id,
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: tags.iter().map(|tag| tag.tag.clone()).collect(),
            created_at: article.created_at.to_string(),
            updated_at: article.updated_at.to_string(),
            favorited: false,
            favorites_count: 0,
            author,
        };

        Ok((StatusCode::CREATED, Json(CreateArticleRes { article })))
    }
}
