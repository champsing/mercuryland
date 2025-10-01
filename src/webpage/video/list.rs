use crate::{
    database::{self, video::Video},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get};

#[get("/api/video/list")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let videos = Video::all(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(videos))
}
