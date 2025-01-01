use google_youtube3::yup_oauth2::ApplicationSecret;
use serde::{Deserialize, Serialize};
use std::{fs, sync::LazyLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub discord_bot_token: String,
    pub discord_channel_id: String,
    pub youtube_channel_id: String,
    pub yt_chat_viewer: ApplicationSecret,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let contents =
        fs::read_to_string("data/config.json").expect("[ERROR] Cannot read config files");
    serde_json::from_str(&contents).expect("[ERROR] Cannot parse config files")
});
