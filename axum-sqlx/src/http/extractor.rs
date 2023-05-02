use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha384;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::http::ApiContext;

pub struct AuthUser {
    pub user_id: Uuid,
}

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);
// const SCHEME_PREFIX: &str = "Token ";

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    /// Standard JWT `exp` claim.
    exp: i64,
}

impl AuthUser {
    pub(in crate::http) fn to_jwt(&self, ctx: &ApiContext) -> String {
        let hmac = Hmac::<Sha384>::new_from_slice(ctx.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        AuthUserClaims {
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }
}
