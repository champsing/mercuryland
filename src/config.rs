// TODO: Use environment variable for discord token and youtube token
use chrono::{DateTime, Utc};
use google_youtube3::yup_oauth2::ApplicationSecret;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::{LazyLock, RwLock},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub wheel_password: String,
    pub discord: DiscordConfig,
    pub youtube_channel_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub exchange: u64,
    pub penalty: u64,
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
