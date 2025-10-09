use crate::{
    database::{self, penalty::Penalty},
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
    pub history: Vec<(i32, NaiveDate)>,
}

#[post("/api/penalty/history/update")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let request = request.into_inner();

    if !auth::verify(&request.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    let mut penalty = match Penalty::by_id(request.id, &transaction)? {
        Some(penalty) => penalty,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    // Validate history: must be sorted by date
    if !request.history.is_empty() {
        for i in 1..request.history.len() {
            if request.history[i - 1].1 > request.history[i].1 {
                return Ok(HttpResponse::BadRequest().finish());
            }
        }
    }

    // Validate that the first history item matches the penalty date
    if let Some((_, first_date)) = request.history.first() {
        if *first_date != penalty.date {
            return Ok(HttpResponse::BadRequest().finish());
        }
    }

    penalty.history = request.history;

    let updated = penalty.update(&transaction)?;
    if updated == 0 {
        transaction.rollback()?;
        return Ok(HttpResponse::NotFound().finish());
    }

    transaction.commit()?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "success": true })))
}