use crate::{config::CONFIG, error::ServerError, youtube::FlowDelegateForDiscord};
use google_youtube3::{
    hyper_rustls, hyper_util,
    yup_oauth2::{self},
    YouTube,
};
use poise;

#[poise::command(slash_command)]
pub async fn link(ctx: super::Context<'_>) -> Result<(), ServerError> {
    let user_id = ctx.author().id.to_string();

    let private_channel =
        serenity::http::Http::create_private_channel(ctx.http(), &serde_json::json!(ctx.author()))
            .await?;

    // construct OAuth2 authenticator
    let auth = yup_oauth2::DeviceFlowAuthenticator::builder(CONFIG.yt_chat_viewer.clone())
        .persist_tokens_to_disk(format!("data/dc_connections/{}.conf", user_id))
        .flow_delegate(Box::new(FlowDelegateForDiscord(private_channel.id.into())))
        .build()
        .await?;

    // construct hyper client
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()?
                .https_only()
                .enable_all_versions()
                .build(),
        );

    let api = YouTube::new(client, auth);

    // triggers the youtube client to get the channel id
    let youtube_channel_id = api
        .channels()
        .list(&vec!["id".into()])
        .mine(true)
        .doit()
        .await?;
    
    println!("{:?}", youtube_channel_id);

    // Here should be the database connection to store the channel id
    // let coin = {
    //     let mut connection = database::get_connection()?;
    //     let transaction = connection.transaction()?;
    // };

    // ctx.say(format!("{}", coin)).await?;
    Ok(())
}
