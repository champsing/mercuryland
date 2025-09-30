use crate::{config::AUTH_CODE, error::ServerError};

use super::{Claims, PRIVATE_KEY};
use actix_web::{HttpResponse, Responder, post, web};
use chrono::{DateTime, Utc};
use jwt::SignWithKey;
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    io::Write,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Deserialize)]
struct Request {
    code: String,
    ip: String,
}

#[derive(Debug, Deserialize)]
struct Logout {
    ip: String,
}

fn struct_claims() -> Claims {
    let now = Duration::as_secs(
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Can't get time"),
    );
    Claims {
        iat: now,
        exp: now + 3600,
    }
}

#[post("/api/auth/login")]
pub async fn login_handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let now = Utc::now();

    let code_consumed = {
        let mut codes = AUTH_CODE
            .write()
            .map_err(|err| ServerError::Internal(format!("auth code lock poisoned: {}", err)))?;
        let mut matched = false;
        codes.retain(|entry| {
            if entry.expires_at <= now {
                return false;
            }
            if !matched && entry.code == request.code {
                matched = true;
                return false;
            }
            true
        });
        matched
    };

    if !code_consumed {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let claims = struct_claims();
    let token = claims.clone().sign_with_key(&*PRIVATE_KEY)?;
    let iat_date_string = DateTime::from_timestamp(claims.clone().iat as i64, 0)
        .expect("Can't get time")
        .to_string();
    let exp_date_string = DateTime::from_timestamp(claims.clone().exp as i64, 0)
        .expect("Can't get time")
        .to_string();
    let log = format!(
        "[Login] Discord code login on {iat} at {ip}, whose session expires on {exp}.",
        iat = iat_date_string,
        ip = request.ip,
        exp = exp_date_string,
    );
    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data/login_history.log");
    writeln!(log_file?, "{log}")?;

    Ok(HttpResponse::Ok().body(token))
}

#[post("/api/auth/logout")]
pub async fn logout_logging(request: web::Json<Logout>) -> String {
    let claims = struct_claims();
    let log = format!(
        "[Login] Discord code logout on {} at {}.",
        claims.clone().iat,
        request.ip,
    );
    log::info!("{}", log);
    log
}
