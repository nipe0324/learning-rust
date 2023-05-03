use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static KEY: [u8; 16] = *include_bytes!("../../dummy.key");
static ONE_DAY: i64 = 60 * 60 * 24; // in seconds

pub fn encode(user_id: Uuid, now: i64) -> Result<String, Error> {
    let claims = Claims::new(user_id, now);

    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(&KEY))
}

// pub fn decode(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
//     jsonwebtoken::decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(&KEY),
//         &Validation::default(),
//     )
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    exp: i64, // Required. Expiration time (as UTC timestamp)
    iat: i64, // Optional. Issured at (as UTC timestamp)
              // Optional
              // aud: String, // Audience (whom token is intended for)
              // iss: String, // Issuer
              // sub: String, // Subject (whom token refers to)
              // nbf: usize, // Not before (as UTC timestamp)
}

impl Claims {
    pub fn new(user_id: Uuid, now: i64) -> Self {
        Self {
            user_id,
            exp: now + ONE_DAY,
            iat: now,
        }
    }
}
