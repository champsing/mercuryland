use crate::{
    database::{self, wheel::Wheel},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get};
use chrono::{TimeDelta, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Response {
    id: u16,
    secret: String,
}

#[get("/api/wheel/create")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let now = Utc::now();

    // only keep records for 7 days
    Wheel::purge(now - TimeDelta::days(7), &transaction)?;
    let w = Wheel::create(now, &transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(Response {
        id: w.id,
        secret: w.secret,
    }))
}
