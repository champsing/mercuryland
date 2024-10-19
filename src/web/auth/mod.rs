pub mod login;
pub mod tick;

use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::LazyLock;

static PRIVATE_KEY: LazyLock<Hmac<Sha256>> = LazyLock::new(|| {
    let mut rng = rand::thread_rng();
    let mut bytes = [0_u8; 32];
    rng.fill_bytes(&mut bytes);
    Hmac::new_from_slice(&bytes).expect("fail to generate HMAC key.")
});

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: u64,
    exp: u64,
}

pub fn verify(token: &str, now: u64) -> bool {
    if let Some(token) = token.verify_with_key(&*PRIVATE_KEY).ok() {
        let token: Token<Header, Claims, _> = token;
        let claims: &Claims = token.claims();
        claims.iat < now && claims.exp > now
    } else {
        false
    }
}
