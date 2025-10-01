use super::{Claims, PRIVATE_KEY, verify};
use crate::error::ServerError;
use actix_web::{HttpResponse, Responder, post, web};
use jwt::SignWithKey;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Request {
    token: String,
}

#[post("/api/auth/tick")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    if verify(&request.token) {
        let claims = Claims {
            iat: now,
            exp: now + 3600,
        };
        let token = claims.sign_with_key(&*PRIVATE_KEY)?;
        Ok(HttpResponse::Ok().body(token))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}
