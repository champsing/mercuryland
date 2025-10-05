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
    pub date: String,
    pub name: String,
    pub detail: String,
    pub state: i32,
    pub history: Vec<(i32, NaiveDate)>,
}

#[post("/api/penalty/insert")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let request = request.into_inner();

    if !auth::verify(&request.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let date = match NaiveDate::parse_from_str(&request.date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let mut penalty = Penalty {
        id: 0,
        date,
        name: request.name,
        detail: request.detail,
        state: PenaltyState::from(request.state),
        history: request.history.into_iter().map(|(state, date)| (PenaltyState::from(state), date)).collect(),
    };

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    penalty.insert(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(penalty))
}