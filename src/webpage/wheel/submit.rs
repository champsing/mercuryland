use crate::{
    database::{self, wheel::Wheel},
    error::ServerError,
};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
// use poise;
// use serenity::all::CreateMessage;
// use serenity::model::id::ChannelId;

#[derive(Debug, Clone, Deserialize)]
struct Request {
    id: u16,
    secret: String,
}

#[post("/wheel/submit")]
pub async fn handler(
    request: web::Json<Request>,
) -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    let w = match Wheel::by_id(request.id, &transaction)? {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some(w) => w,
    };
    if w.secret != request.secret {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // const CHAMPSING: u64 = 1321626646360363018;

    // let message = format!(
    //     "New Wheel session {} has been submitted. Please trigger `/fetch_wheel` command.",
    //     w.id
    // );

    // // not working
    // ChannelId::from(CHAMPSING)
    //     .send_message(&ctx.http, CreateMessage::new().content(message))
    //     .await
    //     .map_err(|_| ServerError::Internal(String::from("Failed to send message")))?;

    Ok(HttpResponse::Ok().finish())
}
