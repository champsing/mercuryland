use actix_web::ResponseError;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum ServerError {
    SystemTime(std::time::SystemTimeError),
    Io(std::io::Error),
    Jwt(jwt::Error),
}

impl ResponseError for ServerError {}
