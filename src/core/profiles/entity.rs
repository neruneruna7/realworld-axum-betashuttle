use sqlx::{prelude::FromRow, types::time::PrimitiveDateTime};
use uuid::Uuid;

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct UserFollowEntity {
    pub id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}
