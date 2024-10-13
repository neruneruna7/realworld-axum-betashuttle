use sqlx::PgPool;
use users::UserDao;

pub mod articles;
pub mod profiles;
pub mod tags;
pub mod users;

#[derive(Clone)]
pub struct Daos {
    pub users: UserDao,
    pub profiles: profiles::ProfileDao,
    pub articles: articles::ArticlesDao,
    pub tags: tags::TagsDao,
}

impl Daos {
    pub fn new(pool: PgPool) -> Self {
        let users = users::UserDao::new(pool.clone());
        let profiles = profiles::ProfileDao::new(pool.clone());
        let articles = articles::ArticlesDao::new(pool.clone());
        let tags = tags::TagsDao::new(pool.clone());
        Self {
            users,
            profiles,
            articles,
            tags,
        }
    }
}
