use itertools::Itertools;
use serenity::all::CreateMessage;
use std::iter::once;

use crate::{
    database::{config::Config, wheel::Wheel},
    discord,
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Request {
    id: u16,
    secret: String,
    password: String,
}

#[post("/api/wheel/submit")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;

    let wheel_password = if let Some(text) = Config::WheelPassword.get(&transaction)? {
        text
    } else {
        return Ok(HttpResponse::ServiceUnavailable().finish());
    };

    let channel_penalty = if let Some(text) = Config::ChannelPenalty.get(&transaction)?
        && let Ok(channel) = text.parse::<u64>()
    {
        channel
    } else {
        return Ok(HttpResponse::ServiceUnavailable().finish());
    };

    if request.password == wheel_password {
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

        discord::Receiver::ChannelId(channel_penalty)
            .message(CreateMessage::new().content(message))
            .await?;

        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}
