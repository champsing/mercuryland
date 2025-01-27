use crate::{
    coin::youtube::Coin,
    database::{self},
    error::ServerError,
};
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Response {
    id: u16,
    coin: i64,
    display_name: String,
}

#[get("/api/leaderboard")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let _transaction = connection.transaction()?;
    let export = Coin::export_all()?;
    println!("{:?}", Coin::export_all());

    Ok(HttpResponse::Ok().json(Response {
        id: serde_json::from_value(export[0].clone())?,
        coin: serde_json::from_value(export[0].clone())?,
        display_name: serde_json::from_value(export[0].clone())?,
    }))
}
