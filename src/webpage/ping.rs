use actix_web::{HttpResponse, Responder, get};
use crate::error::ServerError;

#[get("/api/ping")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "operational"})))
}
