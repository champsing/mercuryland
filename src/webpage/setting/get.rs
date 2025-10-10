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

    if query.id == 0 { // WheelPassword is not retrievable
        return Ok(HttpResponse::Ok().json(serde_json::json!({ "value": "" })))
    }

    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;

    let config = match query.id {
        1 => Config::ChannelPenalty,
        2 => Config::ChannelCoin,
        3 => Config::ChannelVote,
        4 => Config::ChannelAnonymous,
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };
    let value = config.get(&transaction)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "value": value })))

}
