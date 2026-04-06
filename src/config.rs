use chrono::{DateTime, Utc};
use google_youtube3::yup_oauth2::ApplicationSecret;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::LazyLock;
use std::{fs, sync::RwLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub admin: Vec<u64>,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let contents =
        fs::read_to_string("data/config.json").expect("[ERROR] Cannot read config files");
    serde_json::from_str(&contents).expect("[ERROR] Cannot parse config files")
});

// 統一使用 std::env::var 在執行時讀取變數
pub static CFG_DISCORD_TOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var("DISCORD_TOKEN").expect("[ERROR] DISCORD_TOKEN environment variable not set")
});

pub static CFG_YOUTUBE_TOKEN: LazyLock<ApplicationSecret> = LazyLock::new(|| {
    let raw_json =
        env::var("YOUTUBE_TOKEN").expect("[ERROR] YOUTUBE_TOKEN environment variable not set");

    // 建議增加 trim() 以防 GitHub Actions 注入時帶有不可見的換行符
    serde_json::from_str(raw_json.trim()).expect("[ERROR] Cannot parse YOUTUBE_TOKEN")
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
        let token = CFG_DISCORD_TOKEN.clone();
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
