use crate::{
    database::{self, penalty::Penalty},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get};

#[get("/api/penalty/list")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let penalties = Penalty::all(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(penalties))
}