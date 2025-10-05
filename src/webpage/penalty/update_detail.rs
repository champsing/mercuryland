use crate::{
    database::{self, penalty::Penalty},
    error::ServerError,
    webpage::auth,
};
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub token: String,
    pub id: i64,
    pub detail: String,
}

#[post("/api/penalty/detail/update")]
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

    penalty.detail = request.detail;

    let updated = penalty.update(&transaction)?;
    if updated == 0 {
        transaction.rollback()?;
        return Ok(HttpResponse::NotFound().finish());
    }

    transaction.commit()?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "success": true })))
}
