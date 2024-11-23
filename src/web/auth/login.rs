use crate::{config::CONFIG, error::ServerError};

use super::{Claims, PRIVATE_KEY};
use actix_web::{post, web, HttpResponse, Responder};
use jwt::SignWithKey;
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Deserialize)]
struct Request {
    username: String,
    password: String,
}

#[post("/api/auth/login")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    if CONFIG.username == request.username && CONFIG.password == request.password {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let claims = Claims {
            iat: now,
            exp: now + 3600,
        };
        let token = claims.clone().sign_with_key(&*PRIVATE_KEY)?;

        let log = "[Login] User ".to_string()
            + &request.username
            + " loggeed in on "
            + &claims.clone().iat.to_string()
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
