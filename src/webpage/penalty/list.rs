use crate::{
    database::{self, penalty::Penalty},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct PenaltyResponse {
    pub id: i64,
    pub date: NaiveDate,
    pub name: String,
    pub detail: String,
    pub state: i32,
    pub history: Vec<(i32, NaiveDate)>,
}

impl From<Penalty> for PenaltyResponse {
    fn from(penalty: Penalty) -> Self {
        Self {
            id: penalty.id,
            date: penalty.date,
            name: penalty.name,
            detail: penalty.detail,
            state: penalty.state.into(),
            history: penalty.history.into_iter().map(|(state, date)| (state.into(), date)).collect(),
        }
    }
}

#[get("/api/penalty/list")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let penalties = Penalty::all(&transaction)?;
    transaction.commit()?;

    let responses: Vec<PenaltyResponse> = penalties.into_iter().map(PenaltyResponse::from).collect();

    Ok(HttpResponse::Ok().json(responses))
}