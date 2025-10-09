use crate::{database, error::ServerError, webpage::auth};
use actix_web::{HttpResponse, Responder, get, web};
use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub token: String,
}

#[get("/api/setting/backup")]
pub async fn handler(query: web::Query<Query>) -> Result<impl Responder, ServerError> {
    if !auth::verify(&query.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let data = database::backup_database()?;

    let now = Utc::now();
    let filename = format!("sqlite-{}.db", now.format("%Y-%m-%dT%H-%M-%S%.3fZ"));

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(data))
}
