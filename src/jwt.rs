use std::{sync::Arc, time};

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
    state: Arc<AppState>,
}

impl JwtEncoder {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub(crate) fn to_token(&self, user_id: Uuid) -> String {
        let now = chrono::Utc::now();
        let exp = now + DEFALT_SESSION_LEN;
        let claims = Claims {
            exp: exp.timestamp(),
            user_id: user_id,
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.state.jwt_secret.as_ref()),
        )
        .unwrap();
        token
    }
}
