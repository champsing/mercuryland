use crate::{
    coin::youtube::Coin,
    database::{self},
    error::ServerError,
};
use actix_web::{get, HttpResponse, Responder};

#[get("/api/leaderboard")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let export = Coin::all(&transaction)?;
    println!("{:?}", export);

    Ok(HttpResponse::Ok().json(export))
}
