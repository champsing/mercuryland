pub mod google;
pub mod login;
pub mod tick;

use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{
    clone::Clone,
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

static PRIVATE_KEY: LazyLock<Hmac<Sha256>> = LazyLock::new(|| {
    let mut rng = rand::thread_rng();
    let mut bytes = [0_u8; 32];
    rng.fill_bytes(&mut bytes);
    Hmac::new_from_slice(&bytes).expect("fail to generate HMAC key.")
});

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    iat: u64,
    exp: u64,
}

pub fn verify(token: &str) -> bool {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => return false,
    };

    token
        .verify_with_key(&*PRIVATE_KEY)
        .ok()
        .map(|token: Token<Header, Claims, _>| {
            let claims: &Claims = token.claims();
            claims.iat < now && claims.exp > now
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jwt::SignWithKey;

    #[test]
    fn verify_accepts_valid_window() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Can't get time")
            .as_secs();

        let valid_claims = Claims {
            iat: now.saturating_sub(10),
            exp: now + 3600,
        };
        let not_yet_claims = Claims {
            iat: now + 10,
            exp: now + 3600,
        };
        let expired_claims = Claims {
            iat: now.saturating_sub(3600),
            exp: now.saturating_sub(10),
        };

        let valid_token = valid_claims.sign_with_key(&*PRIVATE_KEY).unwrap();
        let not_yet_token = not_yet_claims.sign_with_key(&*PRIVATE_KEY).unwrap();
        let expired_token = expired_claims.sign_with_key(&*PRIVATE_KEY).unwrap();

        assert!(verify(&valid_token));
        assert!(!verify(&not_yet_token));
        assert!(!verify(&expired_token));
    }

    #[test]
    fn verify_rejects_invalid_token() {
        assert!(!verify("invalid"));
    }
}
