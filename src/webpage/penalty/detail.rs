use crate::{
    database::{self, penalty::Penalty},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get, web};

#[get("/api/penalty/detail/{id}")]
pub async fn handler(path: web::Path<i64>) -> Result<impl Responder, ServerError> {
    let id = path.into_inner();

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let penalty = match Penalty::by_id(id, &transaction)? {
        Some(penalty) => penalty,
        None => return Ok(HttpResponse::NotFound().finish()),
    };
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(penalty))
}
