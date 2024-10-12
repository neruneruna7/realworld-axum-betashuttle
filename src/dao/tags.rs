use sqlx::PgPool;

#[derive(Clone)]
pub struct TagsDao {
    pool: PgPool,
}

impl TagsDao {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
