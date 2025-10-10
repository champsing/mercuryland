use crate::{
    database::{self, anonymous::Anonymous},
    error::ServerError,
    webpage::auth,
};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AnonymousWithUser {
    pub id: i64,
    pub author_id: u64,
    pub nickname: String,
    pub avatar: String,
    pub updated_at: DateTime<Utc>,
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

    let cache = crate::webpage::anonymous::ANONYMOUS_CACHE.read().unwrap();
    let mut result = Vec::new();
    for entry in anonymous_entries {
        let author_id = entry.author as u64;
        let (nickname, avatar) = cache.get(&author_id).cloned().unwrap_or_else(|| ("Unknown".to_string(), "".to_string()));
        result.push(AnonymousWithUser {
            id: entry.id,
            author_id,
            nickname,
            avatar,
            updated_at: entry.updated_at,
        });
    }

    Ok(HttpResponse::Ok().json(result))
}
