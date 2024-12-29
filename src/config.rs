use serde::{Deserialize, Serialize};
use std::{fs, sync::LazyLock};
use google_youtube3::yup_oauth2::ApplicationSecret;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub discord: String,
    pub youtube: ApplicationSecret,
    pub oauth_redirect_port: u16
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let contents =
        fs::read_to_string("data/config.json").expect("[ERROR] Cannot read config files");
    serde_json::from_str(&contents).expect("[ERROR] Cannot parse config files")
});
