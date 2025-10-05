use crate::database::{self, penalty::Penalty};
use crate::error::ServerError;
use crate::webpage::auth;
use actix_web::{HttpResponse, Responder, post, web};
use chrono::{NaiveDate, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub token: String,
    pub id: i64,
    pub date: String,
    pub name: String,
    pub state: i32,
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

    if penalty.state < 0 || penalty.state > 4 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    // Clear up any state that is out of order
    penalty.history.retain(|(state, _)| *state <= request.state);
    if let Some((_, initial_date)) = penalty.history.first_mut() {
        // Always update the initial date to the new date
        *initial_date = date;
    };
    if let Some((last_state, _)) = penalty.history.last() {
        // Only add a new history entry if the state has changed
        if *last_state != request.state {
            penalty
                .history
                .push((request.state, Utc::now().date_naive()));
        }
    };

    penalty.date = date;
    penalty.name = request.name;
    penalty.state = request.state;

    let updated = penalty.update(&transaction)?;
    if updated == 0 {
        transaction.rollback()?;
        return Ok(HttpResponse::NotFound().finish());
    }

    transaction.commit()?;

    Ok(HttpResponse::Ok().json(penalty))
}
