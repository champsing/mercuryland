use actix_web::ResponseError;
use derive_more::{Display, From};
use google_youtube3::yup_oauth2;

#[derive(Debug, Display, From)]
pub enum ServerError {
    SystemTime(std::time::SystemTimeError),
    Io(std::io::Error),
    Jwt(jwt::Error),
    Json(serde_json::Error),
    Rusqlite(rusqlite::Error),
    SeaQuery(sea_query::error::Error),
    Serenity(serenity::Error),
    YupOauth2(yup_oauth2::Error),
    Google(google_youtube3::Error),
    Internal(String),
}

impl ResponseError for ServerError {}
