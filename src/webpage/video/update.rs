use crate::{
    database::{self, video::Video},
    error::ServerError,
    webpage::auth,
};
use actix_web::{HttpResponse, Responder, post, web};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub token: String,
    pub id: i64,
    pub date: String,
    pub title: String,
    pub tags: Vec<String>,
    pub duration: String,
}

#[post("/api/video/update")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let request = request.into_inner();

    if !auth::verify(&request.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let date = match NaiveDate::parse_from_str(&request.date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    let mut video = match Video::find_by_id(request.id, &transaction)? {
        Some(video) => video,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    video.date = date;
    video.title = request.title;
    video.tags = request.tags;
    video.duration = request.duration;

    let updated = video.update(&transaction)?;
    if updated == 0 {
        transaction.rollback()?;
        return Ok(HttpResponse::NotFound().finish());
    }

    transaction.commit()?;

    Ok(HttpResponse::Ok().json(video))
}
