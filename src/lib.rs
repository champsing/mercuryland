mod error;
mod config;
mod database;
pub mod web;

use actix_files::NamedFile;
use actix_web::Responder;
use error::ServerError;

pub fn init() -> Result<(), ServerError> {
    database::init()?;
    Ok(())
}

pub async fn index() -> Result<impl Responder, ServerError> {
    NamedFile::open_async("dist/index.html")
        .await
        .map_err(|e| e.into())
}
