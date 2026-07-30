use crate::database::config::Config;
use crate::error::ServerError;
use crate::webpage::auth;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: i32,
    pub value: String,
}

#[post("/api/setting/config")]
pub async fn handler(
    req: HttpRequest,
    request: web::Json<Request>,
) -> Result<impl Responder, ServerError> {
    if !auth::extract_and_verify(&req) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let config = match request.id {
        0 => Config::ChannelPenalty,
        1 => Config::ChannelCoin,
        2 => Config::ChannelVote,
        3 => Config::MessageVote,
        4 => Config::YoutubeChannelId,
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };

    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;
    config.set(request.value.clone(), &transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().finish())
}
