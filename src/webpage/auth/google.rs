use super::{issue_claims, Claims, PRIVATE_KEY};
use crate::{config::CONFIG, error::ServerError};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::{fs::OpenOptions, io::Write};

#[derive(Debug, Deserialize)]
pub struct GoogleLogin {
    credential: String,
    ip: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenInfo {
    aud: String,
    sub: String,
    iss: String,
    exp: String,
    email: Option<String>,
    email_verified: Option<String>,
    name: Option<String>,
    picture: Option<String>,
}

#[derive(Debug, Serialize)]
struct GoogleLoginResponse {
    token: String,
    expires_at: u64,
    sub: String,
    email: Option<String>,
    name: Option<String>,
    picture: Option<String>,
}

fn is_email_verified(flag: &Option<String>) -> bool {
    flag.as_deref()
        .map(|value| value == "true")
        .unwrap_or(false)
}

fn log_login_event(
    claims: &Claims,
    client_ip: Option<&String>,
    google_exp: u64,
) -> Result<(), ServerError> {
    let request_ip = client_ip.cloned().unwrap_or_else(|| "unknown".to_string());
    let issued_at =
        DateTime::<Utc>::from_timestamp(claims.iat as i64, 0).unwrap_or_else(|| Utc::now());
    let expires_at =
        DateTime::<Utc>::from_timestamp(claims.exp as i64, 0).unwrap_or_else(|| Utc::now());
    let google_expires_at =
        DateTime::<Utc>::from_timestamp(google_exp as i64, 0).unwrap_or_else(|| Utc::now());

    let mut message = String::from("[Login][Google] User ");
    if let Some(email) = &claims.email {
        message.push_str(email);
    } else {
        message.push_str(&claims.sub);
    }

    message.push_str(" authenticated at ");
    message.push_str(&issued_at.to_rfc3339());
    message.push_str(" from ");
    message.push_str(&request_ip);
    message.push_str(". Session expires at ");
    message.push_str(&expires_at.to_rfc3339());
    message.push_str(", Google token expires at ");
    message.push_str(&google_expires_at.to_rfc3339());
    message.push('.');

    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data/login_history.log")?;

    writeln!(&mut log_file, "{}", message)?;

    Ok(())
}

#[post("/api/auth/google")]
pub async fn handler(request: web::Json<GoogleLogin>) -> Result<impl Responder, ServerError> {
    let credential = request.credential.trim();
    if credential.is_empty() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let client = reqwest::Client::new();
    let response = client
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&[("id_token", credential)])
        .send()
        .await?;

    if !response.status().is_success() {
        log::warn!(
            "Google token validation failed with status {}",
            response.status()
        );
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let body = response.bytes().await?;
    let token_info: GoogleTokenInfo = from_slice(&body)?;

    if token_info.aud != CONFIG.google_sso.client_id {
        log::warn!(
            "Audience mismatch: expected {}, got {}",
            CONFIG.google_sso.client_id,
            token_info.aud
        );
        return Ok(HttpResponse::Unauthorized().finish());
    }

    match token_info.iss.as_str() {
        "https://accounts.google.com" | "accounts.google.com" => {}
        issuer => {
            log::warn!("Unexpected issuer {}", issuer);
            return Ok(HttpResponse::Unauthorized().finish());
        }
    }

    if !is_email_verified(&token_info.email_verified) {
        log::warn!("Email for subject {} is not verified", token_info.sub);
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let google_exp = token_info
        .exp
        .parse::<u64>()
        .map_err(|_| ServerError::Internal("Failed to parse Google token expiry".into()))?;

    let claims = issue_claims(
        token_info.sub.clone(),
        token_info.email.clone(),
        token_info.name.clone(),
        token_info.picture.clone(),
    );

    let token = claims.clone().sign_with_key(&*PRIVATE_KEY)?;

    if let Err(err) = log_login_event(&claims, request.ip.as_ref(), google_exp) {
        log::error!("Failed to log Google login event: {}", err);
    }

    let response = GoogleLoginResponse {
        token,
        expires_at: claims.exp,
        sub: claims.sub.clone(),
        email: claims.email.clone(),
        name: claims.name.clone(),
        picture: claims.picture.clone(),
    };

    Ok(HttpResponse::Ok().json(response))
}
