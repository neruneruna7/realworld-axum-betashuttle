use sqlx::{PgPool, Pool};

pub mod profiles;
pub mod users;

#[derive(Clone)]
pub struct Daos {
    pub users: UsersIntermediateDao,
}

impl Daos {
    pub fn new(pool: PgPool) -> Self {
        let users = UsersIntermediateDao::new(pool);
        Self { users }
    }
}

/// ユーザーハンドラとDAOの間に，多対多の関係を解決する中間層
#[derive(Clone)]
pub struct UsersIntermediateDao {
    pub users: users::UserDao,
}

impl UsersIntermediateDao {
    pub fn new(pool: PgPool) -> Self {
        let users = users::UserDao::new(pool);
        Self { users }
    }
}

// pub struct PrifilesIntermediateDao {
//     pub profiles: users::ProfileDao,
// }
