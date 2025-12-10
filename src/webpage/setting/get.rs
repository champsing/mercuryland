use crate::database::config::Config;
use crate::error::ServerError;
use crate::webpage::auth;
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub token: String,
    pub id: i32,
}

#[get("/api/setting/config")]
pub async fn handler(query: web::Query<Query>) -> Result<impl Responder, ServerError> {
    if !auth::verify(&query.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;

    let config = match query.id {
        0 => Config::ChannelPenalty,
        1 => Config::ChannelCoin,
        2 => Config::ChannelVote,
        3 => Config::MessageVote,
        4 => Config::YoutubeChannelId,
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };
    let value = config.get(&transaction)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "value": value })))
}
