mod coin;
mod link;
mod wheel;

use once_cell::sync::OnceCell as OnceLock;

use crate::{config::CONFIG, error::ServerError};
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use poise::{self};
use serde_json::json;
use serenity::all::{ChannelId, CreateAttachment, Http};
use std::collections::HashMap;
use std::sync::Arc;

type Data = ();
type Context<'a> = poise::Context<'a, Data, ServerError>;

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();

pub async fn send_text(channel_id: ChannelId, text: &String) -> Result<(), ServerError> {
    send_message(channel_id, vec![], &json!({"content": text})).await?;
    Ok(())
}

pub async fn send_message(
    channel_id: ChannelId,
    files: Vec<CreateAttachment>,
    map: &impl serde::Serialize,
) -> Result<(), ServerError> {
    HTTP.wait().send_message(channel_id, files, map).await?;
    Ok(())
}

pub async fn run() -> Result<(), ServerError> {
    let options = poise::FrameworkOptions {
        commands: vec![
            poise::Command {
                name: String::from("fetch_wheel"),
                description: Some(String::from(
                    "Fetch the text in the Drawn Zone of the specified wheel",
                )),
                description_localizations: HashMap::from([(
                    "zh-TW".to_string(),
                    String::from("獲取輪盤抽中區"),
                )]),
                ..wheel::fetch_wheel()
            },
            poise::Command {
                name: String::from("coin"),
                description: Some(String::from("Query your Mecury Coin balance")),
                description_localizations: HashMap::from([(
                    "zh-TW".to_string(),
                    String::from("查詢水星幣餘額"),
                )]),
                ..coin::coin()
            },
            poise::Command {
                name: String::from("link"),
                description: Some(String::from(
                    "Link your Discord account to your YouTube channel record",
                )),
                description_localizations: HashMap::from([(
                    "zh-TW".to_string(),
                    String::from("連結 Discord 帳號至 YouTube 頻道"),
                )]),
                ..link::link()
            },
            poise::Command {
                name: String::from("unlink"),
                description: Some(String::from(
                    "Unlink your Discord account and your YouTube channel record",
                )),
                description_localizations: HashMap::from([(
                    "zh-TW".to_string(),
                    String::from("斷開 YouTube 頻道與 Discord 帳號的連結"),
                )]),
                ..link::unlink()
            },
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .options(options)
        .build();

    let mut client = ClientBuilder::new(
        &CONFIG.discord_bot_token,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await?;

    HTTP.get_or_init(|| client.http.clone());
    client.start().await?;

    Ok(())
}
