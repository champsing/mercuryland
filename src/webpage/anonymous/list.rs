use crate::{
    database::{self, anonymous::Anonymous},
    error::ServerError,
    webpage::auth,
};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub token: String,
}

#[get("/api/anonymous/list")]
pub async fn handler(query: web::Query<Query>) -> Result<impl Responder, ServerError> {
    if !auth::verify(&query.token) {
        return Ok(HttpResponse::Forbidden().finish());
    }
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let anonymous_entries = Anonymous::all(&transaction)?;
    transaction.commit()?;

    Ok(HttpResponse::Ok().json(anonymous_entries))
}
