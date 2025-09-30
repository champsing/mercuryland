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
    pub date: String,
    pub link: String,
    pub title: String,
    pub tags: Vec<String>,
    pub duration: String,
}

#[post("/api/video/insert")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let request = request.into_inner();

    if !auth::verify(&request.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let date = match NaiveDate::parse_from_str(&request.date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let mut video = Video {
        id: None,
        date,
        link: request.link,
        title: request.title,
        tags: request.tags,
        duration: request.duration,
    };

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    if Video::find_by_link(&video.link, &transaction)?.is_some() {
        transaction.rollback()?;
        return Ok(HttpResponse::Conflict().finish());
    }

    video.insert(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(video))
}
