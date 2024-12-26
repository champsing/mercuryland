use crate::{
    database::{self, wheel::Wheel},
    error::ServerError,
};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Request {
    id: u16,
    secret: String,
    content: Vec<String>,
}

#[post("/wheel/update")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    let mut w = match Wheel::by_id(request.id, &transaction)? {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some(w) => w,
    };
    if w.secret != request.secret {
        return Ok(HttpResponse::Forbidden().finish());
    }

    w.updated_at = Utc::now();
    w.content = serde_json::to_value(request.content.clone())?;
    w.update(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().finish())
}
