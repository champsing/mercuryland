use std::iter::once;
use itertools::Itertools;

use crate::{
    database::{self, wheel::Wheel},
    discord,
    error::ServerError,
};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Request {
    id: u16,
    secret: String,
}

#[post("/api/wheel/submit")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    const PENALTY_CHANNEL_ID: u64 = 967078051714326558; // 惡靈懲罰

    let w = match Wheel::by_id(request.id, &transaction)? {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some(w) => w,
    };
    if w.secret != request.secret {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let (time, content): (_, Vec<String>) = (
        w.updated_at.timestamp(),
        serde_json::from_value(w.content).expect("Can't find the content"),
    );

    let content = content
        .into_iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i, s));

    let message = once(format!("<t:{}:D>", time)).chain(content).join("\n");

    discord::send_message(PENALTY_CHANNEL_ID.into(), vec![], &serde_json::json!({"content": message})).await?;

    Ok(HttpResponse::Ok().finish())
}
