use std::time;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

const DEFALT_SESSION_LEN: time::Duration = time::Duration::from_secs(60 * 60 * 2);
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub user_id: Uuid,
}

pub struct JwtEncoder {
    user_id: Uuid,
}

impl JwtEncoder {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }

    pub fn to_token(&self, state: &AppState) -> String {
        let now = chrono::Utc::now();
        let exp = now + DEFALT_SESSION_LEN;
        let claims = Claims {
            exp: exp.timestamp(),
            user_id: self.user_id,
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(state.jwt_secret.as_ref()),
        )
        .unwrap();
        token
    }
}
