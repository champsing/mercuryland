use crate::{
    database::{self, user::User},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get};

#[get("/api/leaderboard")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let export = User::all(&transaction)?;
    // println!("{:?}", export);
    Ok(HttpResponse::Ok().json(export))
}
