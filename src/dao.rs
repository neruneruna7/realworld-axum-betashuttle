use sqlx::{PgPool, Pool};
use users::UserDao;

pub mod profiles;
pub mod users;

#[derive(Clone)]
pub struct Daos {
    pub users: UserDao,
}

impl Daos {
    pub fn new(pool: PgPool) -> Self {
        let users = users::UserDao::new(pool);
        Self { users }
    }
}
