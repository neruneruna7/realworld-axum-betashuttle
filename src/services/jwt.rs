use std::time;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ArcState;

const DEFAULT_SESSION_LEN: time::Duration = time::Duration::from_secs(60 * 60 * 2);
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub user_id: Uuid,
}

pub struct JwtService {
    state: ArcState,
}

impl JwtService {
    pub(crate) fn new(state: ArcState) -> Self {
        Self { state }
    }

    pub(crate) fn to_token(&self, user_id: Uuid) -> String {
        let now = chrono::Utc::now();
        let exp = now + DEFAULT_SESSION_LEN;
        let claims = Claims {
            exp: exp.timestamp(),
            user_id,
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.state.jwt_secret.as_ref()),
        )
        .unwrap();
        token
    }

    pub(crate) fn get_claim_from_token(
        &self,
        token: &str,
    ) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.state.jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        );
        token.map(|data| data.claims)
    }
}
