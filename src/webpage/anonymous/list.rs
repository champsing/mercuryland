use crate::{
    config::CFG_DISCORD_TOKEN,
    database::{self, anonymous::Anonymous},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;
use serenity::http::Http;
use serenity::model::id::UserId;

#[derive(Serialize)]
struct AnonymousResponse {
    id: i64,
    author: String, // username instead of ID
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[get("/api/anonymous/list")]
pub async fn handler() -> Result<impl Responder, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let anonymous_entries = Anonymous::all(&transaction)?;
    transaction.commit()?;

    // Create HTTP client for Discord API
    let http = Http::new(&CFG_DISCORD_TOKEN);

    // Convert Discord IDs to usernames
    let mut responses = Vec::new();
    for entry in anonymous_entries {
        let username = match http.get_user(UserId::from(entry.author as u64)).await {
            Ok(user) => format!("{} ({})", user.name, entry.author),
            Err(_) => format!("Unknown User ({})", entry.author),
        };
        responses.push(AnonymousResponse {
            id: entry.id,
            author: username,
            updated_at: entry.updated_at,
        });
    }

    Ok(HttpResponse::Ok().json(responses))
}