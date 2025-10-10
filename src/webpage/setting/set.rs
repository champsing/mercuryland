use crate::database::config::Config;
use crate::error::ServerError;
use crate::webpage::auth;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub token: String,
    pub id: i32,
    pub value: String,
}

#[post("/api/setting/config")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    if !auth::verify(&request.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let config = match request.id {
        0 => Config::WheelPassword,
        1 => Config::ChannelPenalty,
        2 => Config::ChannelCoin,
        3 => Config::ChannelVote,
        4 => Config::ChannelAnonymous,
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };

    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;
    config.set(request.value.clone(), &transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().finish())
}
