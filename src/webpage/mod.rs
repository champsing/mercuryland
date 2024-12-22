pub mod ping;
pub mod wheel;
pub mod auth;

use actix_files::NamedFile;
use actix_web::Responder;
use crate::error::ServerError;

pub async fn index() -> Result<impl Responder, ServerError> {
    NamedFile::open_async("dist/index.html")
        .await
        .map_err(|e| e.into())
}
