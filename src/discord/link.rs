use std::sync::RwLock;

use crate::{
    coin::youtube::Coin,
    config::CONFIG,
    database::{self},
    discord,
    error::ServerError,
    youtube::FlowDelegateForDiscord,
};
use chrono::Utc;
use google_youtube3::{
    hyper_rustls,
    hyper_util::{self},
    yup_oauth2, YouTube,
};
use poise;
use serenity::{builder::CreateMessage, model::channel::MessageFlags};

#[poise::command(slash_command)]
pub async fn link(ctx: super::Context<'_>) -> Result<(), ServerError> {
    let _check_exist = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        let _ = match Coin::by_discord(ctx.author().id.get().to_string(), &transaction) {
            Ok(..) => {
                drop(transaction);
                return discord::send_message(ctx.channel_id(), vec![], &CreateMessage::new().content("您已經將本帳號連結到某個 YouTube 頻道了。\n如要綁定其他 YouTube 頻道，請先解除連結或使用不同 Discord 帳號。").flags(MessageFlags::EPHEMERAL)).await;
            }
            Err(..) => false,
        };
    };

    let private_channel = ctx.author().create_dm_channel(ctx).await?;

    // if the author hasn't completed OAuth2 yet
    ctx.author().dm(ctx, CreateMessage::new().content("您正在執行的操作是授權我們存取您的 Google 帳號以讀取您帳號旗下的 YouTube 頻道，用於將您的 Discord 帳號與 YouTube 頻道建立內部資料庫連結。")).await?;

    let auth = yup_oauth2::DeviceFlowAuthenticator::builder(CONFIG.dcyt_link.clone())
        .persist_tokens_to_disk(format!("data/dc_connects/{}.conf", ctx.author().id))
        .flow_delegate(Box::new(FlowDelegateForDiscord(private_channel.id.into())))
        .build()
        .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()?
                .https_only()
                .enable_all_versions()
                .build(),
        );
    let api = YouTube::new(client, auth);
    // necessary to trigger auth flow
    let yt_ch_list = api
        .channels()
        .list(&vec!["id".into()])
        .mine(true)
        .doit()
        .await?;

    let channel = match yt_ch_list.1.items {
        Some(c) => match c[0].id.clone() {
            Some(id) => id,
            None => return discord::send_message(ctx.channel_id(), vec![], &CreateMessage::new().content("找不到您帳號中的 YouTube 頻道。\n請先於 YouTube 建立頻道，或更換其他 Google 帳號再試一次。").flags(MessageFlags::EPHEMERAL)).await
        },
        None => return discord::send_message(ctx.channel_id(), vec![], &CreateMessage::new().content("找不到您帳號中的 YouTube 頻道。\n請先於 YouTube 建立頻道，或更換其他 Google 帳號再試一次。").flags(MessageFlags::EPHEMERAL)).await
    };

    println!("{:?}", channel);

    // Here should be the database connection to store the channel id
    let _ = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;

        let _ = match Coin::by_youtube(channel, &transaction)? {
            Some(mut r) => {
                r.discord_id = ctx.author().id.to_string();
                r.updated_at = Utc::now();
                r.update(&transaction)?;
                transaction.commit()?;
            }
            None => {
                return Err(String::from(
                    "YouTube 頻道不存在，請先在直播聊天室留言以建立您的 YouTube 頻道記錄。",
                )
                .into())
            }
        };
    };

    ctx.say(format!("{}", "您已成功連結您的帳號至 YouTube 頻道"))
        .await?;
    Ok(())
}
