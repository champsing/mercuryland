mod coin;
mod wheel;

use std::sync::OnceLock;

use crate::{config::CONFIG, error::ServerError};
use poise;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::all::{ChannelId, CreateAttachment, Http};
use std::sync::Arc;

type Data = ();
type Context<'a> = poise::Context<'a, Data, ServerError>;

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();

// untested
pub async fn send_message(
    channel_id: ChannelId,
    files: Vec<CreateAttachment>,
    map: &impl serde::Serialize,
) -> Result<(), ServerError> {
    if let Some(http) = HTTP.get() {
        http.send_message(channel_id, files, map).await?;
    }

    Ok(())
}

pub async fn run() -> Result<(), ServerError> {
    let options = poise::FrameworkOptions {
        commands: vec![wheel::fetch_wheel(), coin::coin()],
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
        &CONFIG.discord,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await?;

    HTTP.get_or_init(|| client.http.clone());
    client.start().await?;

    Ok(())
}
