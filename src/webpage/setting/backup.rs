use crate::{database, error::ServerError, webpage::auth};
use actix_web::{HttpRequest, HttpResponse, Responder, get};
use chrono::Utc;

#[get("/api/setting/backup")]
pub async fn handler(req: HttpRequest) -> Result<impl Responder, ServerError> {
    if !auth::extract_and_verify(&req) {
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
