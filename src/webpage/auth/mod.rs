pub mod google;
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
pub(super) struct Claims {
    pub(super) sub: String,
    pub(super) email: Option<String>,
    pub(super) name: Option<String>,
    pub(super) picture: Option<String>,
    iat: u64,
    exp: u64,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't get time")
        .as_secs()
}

pub(super) fn verify(token: &str, now: u64) -> Option<Claims> {
    if let Some(token) = token.verify_with_key(&*PRIVATE_KEY).ok() {
        let token: Token<Header, Claims, _> = token;
        let claims: &Claims = token.claims();
        if claims.iat < now && claims.exp > now {
            Some(claims.clone())
        } else {
            None
        }
    } else {
        None
    }
}

pub(super) fn issue_claims(
    sub: String,
    email: Option<String>,
    name: Option<String>,
    picture: Option<String>,
) -> Claims {
    let now = now_secs();

    Claims {
        sub,
        email,
        name,
        picture,
        iat: now,
        exp: now + 3600,
    }
}

pub(super) fn refresh_claims(previous: &Claims) -> Claims {
    let now = now_secs();

    Claims {
        sub: previous.sub.clone(),
        email: previous.email.clone(),
        name: previous.name.clone(),
        picture: previous.picture.clone(),
        iat: now,
        exp: now + 3600,
    }
}
