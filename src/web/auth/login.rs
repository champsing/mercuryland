use crate::{config::CONFIG, error::ServerError};

use super::{Claims, PRIVATE_KEY};
use actix_web::{post, web, HttpResponse, Responder};
use jwt::SignWithKey;
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    io::Write,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Deserialize)]
struct Request {
    username: String,
    password: String,
    ip: String,
}

#[derive(Debug, Deserialize)]
struct Logout {
    username: String,
    ip: String,
}

fn struct_claims() -> Claims {
    let now = Duration::as_secs(
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Can't get time"),
    );
    return Claims {
        iat: now,
        exp: now + 3600,
    };
}

#[post("/api/auth/login")]
pub async fn login_handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    if CONFIG.username == request.username && CONFIG.password == request.password {
        let claims = struct_claims();
        let token = claims.clone().sign_with_key(&*PRIVATE_KEY)?;

        let log = "[Login] User ".to_string()
            + &request.username
            + " logged in on "
            + &claims.clone().iat.to_string()
            + " at "
            + &request.ip
            + ", whose session expires on "
            + &claims.clone().exp.to_string()
            + ".";
        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("data/login_history.log");
        writeln!(log_file?, "{log}")?;

        Ok(HttpResponse::Ok().body(token))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[post("/api/auth/logout")]
pub async fn logout_logging(request: web::Json<Logout>) -> String {
    let claims = struct_claims();
    let log = "[Login] User ".to_string()
        + &request.username
        + " logged out on "
        + &claims.clone().iat.to_string()
        + " at "
        + &request.ip
        + ".";
    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data/login_history.log");
    writeln!(log_file.expect("Can't find log file."), "{log}").expect("Can't write log.");
    return log;
}
