pub mod wheel;

use crate::{config::CONFIG, error::ServerError};
use poise;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};

type Data = ();
type Context<'a> = poise::Context<'a, Data, ServerError>;

pub async fn run() -> Result<(), ServerError> {
    let options = poise::FrameworkOptions {
        commands: vec![wheel::fetch_wheel()],
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
    client.start().await?;

    Ok(())
}
