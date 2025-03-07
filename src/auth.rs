use anyhow::Result;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub game_id: String,
    pub user_id: String,
    pub admin: bool,
    pub exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: i64, // Optional. Issued at (as UTC timestamp)
                  // aud: String, // Optional. Audience
                  // iss: String, // Optional. Issuer
                  // nbf: usize, // Optional. Not Before (as UTC timestamp)
                  // sub: String, // Optional. Subject (whom token refers to)
}

impl Default for Claims {
    fn default() -> Self {
        let iat = chrono::Utc::now();
        let exp = iat + Duration::days(30);
        Self {
            game_id: Default::default(),
            user_id: Default::default(),
            admin: false,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}

impl Claims {
    pub fn sign(&self) -> Result<String> {
        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret("TODO_UPDATE_ME".as_ref()),
        )?)
    }

    pub fn decode(token: String) -> Result<Self> {
        let data = decode::<Self>(
            &token,
            &DecodingKey::from_secret("TODO_UPDATE_ME".as_ref()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }

    pub fn has_access_to_room(&self, room_id: &str) -> bool {
        self.game_id == room_id
    }
}
