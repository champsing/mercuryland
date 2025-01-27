use crate::{
    coin::youtube::Coin, database::{self, wheel::Wheel}, error::ServerError
};
use actix_web::{dev::Server, get, HttpResponse, Responder};
use chrono::{TimeDelta, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Response {
    id: u16,
    coin: i64,
    display_name: String,
}

#[get("/api/leaderboard")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    // let mut connection = database::get_connection()?;
    // let transaction = connection.transaction()?;
    // let export = Coin::export_all();
    // println!("{:?}", Coin::export_all());
    
    // Ok(HttpResponse::Ok().json(Response {
    //     id: export.id,
    //     coin: export.coin,
    //     display_name: export.display
    // }))
    Ok(HttpResponse::Ok())
}