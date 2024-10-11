use sqlx::PgPool;
use users::UserDao;

pub mod articles;
pub mod profiles;
pub mod users;

#[derive(Clone)]
pub struct Daos {
    pub users: UserDao,
    pub profiles: profiles::ProfileDao,
    pub articles: articles::ArticlesDao,
}

impl Daos {
    pub fn new(pool: PgPool) -> Self {
        let users = users::UserDao::new(pool.clone());
        let profiles = profiles::ProfileDao::new(pool.clone());
        let articles = articles::ArticlesDao::new(pool);
        Self {
            users,
            profiles,
            articles,
        }
    }
}
