use crate::database::{self, penalty::Penalty};
use crate::error::ServerError;
use crate::webpage::auth;
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

    let history = if request.state == 0 {
        vec![(0, date)]
    } else if request.state == 1 {
        vec![(0, date), (1, date)]
    } else {
        return Ok(HttpResponse::BadRequest().finish());
    };

    let mut penalty = Penalty {
        id: 0,
        date,
        name: request.name,
        detail: request.detail,
        state: request.state,
        history,
    };

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    penalty.insert(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(penalty))
}
