use crate::error::ServerError;
use actix_web::{HttpResponse, Responder, get};

#[get("/api/ping")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "operational", "version": env!("CARGO_PKG_VERSION")})))
}
