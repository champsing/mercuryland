pub mod list;

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

pub static ANONYMOUS_CACHE: LazyLock<RwLock<HashMap<u64, (String, String)>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

use crate::config::CFG_DISCORD_TOKEN;
use crate::database;
use crate::error::ServerError;
use serenity::http::Http;
use serenity::model::id::UserId;

pub async fn update_cache() -> Result<(), ServerError> {
    let http = Http::new(&CFG_DISCORD_TOKEN);
    loop {
        // Get all anonymous entries and collect authors
        let authors: std::collections::HashSet<u64> = {
            let mut connection = database::get_connection()?;
            let transaction = connection.transaction()?;
            let anonymous_entries = crate::database::anonymous::Anonymous::all(&transaction)?;
            transaction.commit()?;

            let mut authors = std::collections::HashSet::new();
            for entry in &anonymous_entries {
                authors.insert(entry.author as u64);
            }
            authors
        };

        // Collect users to insert
        let mut to_insert = Vec::new();
        for author_id in authors {
            {
                let cache = ANONYMOUS_CACHE.read().unwrap();
                if cache.contains_key(&author_id) {
                    continue;
                }
            } // drop read guard
            // Fetch user
            match http.get_user(UserId::from(author_id)).await {
                Ok(user) => {
                    let nickname = user
                        .global_name
                        .clone()
                        .unwrap_or_else(|| user.name.clone());
                    let avatar = user.avatar_url().unwrap_or_else(|| "".to_string());
                    to_insert.push((author_id, (nickname, avatar)));
                }
                Err(e) => {
                    eprintln!("Failed to fetch user {}: {}", author_id, e);
                    to_insert.push((author_id, ("Unknown".to_string(), "".to_string())));
                }
            }
        }

        // Insert into cache
        {
            let mut cache = ANONYMOUS_CACHE.write().unwrap();
            for (author_id, data) in to_insert {
                cache.insert(author_id, data);
            }
        } // drop write guard

        // Sleep for 30 minutes
        tokio::time::sleep(tokio::time::Duration::from_secs(1800)).await;
    }
}
