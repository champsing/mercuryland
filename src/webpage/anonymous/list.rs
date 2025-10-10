use crate::{
    config::CFG_DISCORD_TOKEN,
    database::{self, anonymous::Anonymous},
    error::ServerError,
    webpage::auth,
};
use actix_web::{HttpResponse, Responder, get, web};
use chrono::{DateTime, Duration, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serenity::http::Http;
use serenity::model::id::UserId;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

static USER_CACHE: Lazy<Arc<Mutex<HashMap<u64, (String, DateTime<Utc>)>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Serialize)]
struct AnonymousResponse {
    id: i64,
    author: String, // username instead of ID
    updated_at: chrono::DateTime<chrono::Utc>,
}

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

    // Get unique author IDs
    let unique_ids: HashSet<u64> = anonymous_entries.iter().map(|e| e.author as u64).collect();

    // Create HTTP client for Discord API
    let http = Http::new(&CFG_DISCORD_TOKEN);

    // Populate cache for missing users
    {
        let mut cache = USER_CACHE.lock().unwrap();
        for &id in &unique_ids {
            if !cache.contains_key(&id) || (Utc::now() - cache[&id].1) > Duration::hours(12) {
                let username = match http.get_user(UserId::from(id)).await {
                    Ok(user) => format!("{} ({})", user.name, id),
                    Err(_) => format!("Unknown User ({})", id),
                };
                cache.insert(id, (username, Utc::now()));
            }
        }
    }

    // Build responses using cache
    let mut responses = Vec::new();
    for entry in anonymous_entries {
        let (username, _) = USER_CACHE.lock().unwrap().get(&(entry.author as u64)).cloned().unwrap();
        responses.push(AnonymousResponse {
            id: entry.id,
            author: username,
            updated_at: entry.updated_at,
        });
    }

    Ok(HttpResponse::Ok().json(responses))
}
