use crate::{
    database::{self, video::Video},
    error::ServerError,
    webpage::auth,
};
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub token: String,
    pub id: i64,
}

#[post("/api/video/delete")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let request = request.into_inner();

    if !auth::verify(&request.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    let video = match Video::by_id(request.id, &transaction)? {
        Some(video) => video,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let deleted = video.delete(&transaction)?;

    if deleted == 0 {
        transaction.rollback()?;
        return Ok(HttpResponse::NotFound().finish());
    }

    transaction.commit()?;

    Ok(HttpResponse::Ok().finish())
}
