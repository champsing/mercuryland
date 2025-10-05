use crate::{
    database::{self, penalty::{Penalty, PenaltyState}},
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
    pub name: String,
    pub detail: String,
    pub state: PenaltyState,
    pub history: Vec<(PenaltyState, NaiveDate)>,
}

#[post("/api/penalty/update")]
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

    let mut penalty = match Penalty::by_id(request.id, &transaction)? {
        Some(penalty) => penalty,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    penalty.date = date;
    penalty.name = request.name;
    penalty.detail = request.detail;
    penalty.state = request.state;
    penalty.history = request.history;

    let updated = penalty.update(&transaction)?;
    if updated == 0 {
        transaction.rollback()?;
        return Ok(HttpResponse::NotFound().finish());
    }

    transaction.commit()?;

    Ok(HttpResponse::Ok().json(penalty))
}