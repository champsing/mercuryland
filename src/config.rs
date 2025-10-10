use chrono::{DateTime, Utc};
use google_youtube3::yup_oauth2::ApplicationSecret;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::{LazyLock, RwLock},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub youtube_channel_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub exchange: u64,
    pub admin: Vec<u64>,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let contents =
        fs::read_to_string("data/config.json").expect("[ERROR] Cannot read config files");
    serde_json::from_str(&contents).expect("[ERROR] Cannot parse config files")
});

pub static CFG_DISCORD_TOKEN: LazyLock<&str> = LazyLock::new(|| env!("DISCORD_TOKEN"));

pub static CFG_YOUTUBE_TOKEN: LazyLock<ApplicationSecret> = LazyLock::new(|| {
    serde_json::from_str(env!("YOUTUBE_TOKEN")).expect("[ERROR] Cannot parse YOUTUBE_TOKEN")
});

#[derive(Debug, Clone)]
pub struct AuthCode {
    pub code: String,
    pub expires_at: DateTime<Utc>,
}

pub static AUTH_CODE: LazyLock<RwLock<Vec<AuthCode>>> = LazyLock::new(|| RwLock::new(Vec::new()));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discord_token_structure() {
        let token = *CFG_DISCORD_TOKEN;
        assert!(!token.is_empty(), "DISCORD_TOKEN should not be empty");

        let parts: Vec<&str> = token.split('.').collect();
        assert!(
            parts.len() == 3,
            "Discord token should contain three sections"
        );

        for part in &parts {
            assert!(
                !part.is_empty(),
                "Each segment of the token should be non-empty"
            );

            assert!(
                part.chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
                "Each segment of the token should contain only letters, digits, hyphens, or underscores"
            );
        }
    }

    #[test]
    fn test_youtube_token_parses() {
        // Accessing CFG_YOUTUBE_TOKEN will initialize it and panic if YOUTUBE_TOKEN is not set or invalid JSON.
        let _token = &*CFG_YOUTUBE_TOKEN;
        // Reaching this point means the token was set and parsed successfully.
    }
}
