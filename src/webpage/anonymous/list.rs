use crate::{
    database::{self, anonymous::Anonymous},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get};

#[get("/api/anonymous/list")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let anonymous = Anonymous::all(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(anonymous))
}