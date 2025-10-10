use crate::database::config::Config;
use crate::error::ServerError;
use crate::webpage::auth;
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub token: String,
    pub id: Option<i32>,
}

#[get("/api/setting/config")]
pub async fn handler(query: web::Query<Query>) -> Result<impl Responder, ServerError> {
    if !auth::verify(&query.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;

    if let Some(id) = query.id {
        let config = match id {
            1 => Config::ChannelPenalty,
            2 => Config::ChannelCoin,
            3 => Config::ChannelVote,
            4 => Config::ChannelAnonymous,
            _ => return Ok(HttpResponse::BadRequest().finish()),
        };
        let value = config.get(&transaction)?;
        Ok(HttpResponse::Ok().json(serde_json::json!({ "id": id, "value": value })))
    } else {
        let configs = vec![
            (0, Config::WheelPassword.get(&transaction)?),
            (1, Config::ChannelPenalty.get(&transaction)?),
            (2, Config::ChannelCoin.get(&transaction)?),
            (3, Config::ChannelVote.get(&transaction)?),
            (4, Config::ChannelAnonymous.get(&transaction)?),
        ];
        Ok(HttpResponse::Ok().json(configs))
    }
}
