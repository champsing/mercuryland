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
    YouTube, hyper_rustls,
    hyper_util::{self},
    yup_oauth2,
};
use poise::{self, reply::CreateReply, serenity_prelude::CreateMessage};

async fn check_exist(context: super::Context<'_>) -> Result<bool, ServerError> {
    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let existence = match Coin::by_discord(context.author().id.get().to_string(), &transaction) {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => return Err(String::from("Account already linked").into()),
    };
    Ok(existence)
}

#[poise::command(slash_command)]
pub async fn link(ctx: super::Context<'_>) -> Result<(), ServerError> {
    let author = ctx.author();

    if check_exist(ctx).await? == true {
        ctx.send(
            CreateReply::default()
            .content(format!("您已經將本帳號連結到某個 YouTube 頻道了。\n如要綁定其他 YouTube 頻道，請先使用 {} 解除連結或使用不同 Discord 帳號。", CONFIG.slash_command_strings.unlink))
            .ephemeral(true)
        ).await?;
        return Ok(());
    }

    ctx.reply("現在開始連結 Discord 帳號至 YouTube 頻道。請依照私訊說明操作。")
        .await?;

    // if the author hasn't completed OAuth2 yet
    author.dm(ctx, CreateMessage::new().content("您正在執行的操作是授權我們存取您的 Google 帳號以讀取您帳號旗下的 YouTube 頻道，用於將您的 Discord 帳號與 YouTube 頻道建立內部資料庫連結。")).await?;

    let auth = yup_oauth2::DeviceFlowAuthenticator::builder(CONFIG.dcyt_link.clone())
        .flow_delegate(Box::new(FlowDelegateForDiscord(discord::Receiver::UserId(
            author.id.get(),
        ))))
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
            None => {
                ctx.send(
                    CreateReply::default()
                    .content("找不到您帳號中的 YouTube 頻道。\n請先於 YouTube 建立頻道，或更換其他 Google 帳號再試一次。")
                    .ephemeral(true)
                ).await?;
                return Ok(());
            }
        },
        None => {
            ctx.send(
                CreateReply::default()
                .content("找不到您帳號中的 YouTube 頻道。\n請先於 YouTube 建立頻道，或更換其他 Google 帳號再試一次。")
                .ephemeral(true)
            ).await?;
            return Ok(());
        }
    };

    let mut no_channel_found = false;

    // Here should be the database connection to store the channel id
    let _ = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;

        let _ = match Coin::by_youtube(channel, &transaction) {
            Ok(Some(mut r)) => {
                r.discord_id = author.id.get();
                r.updated_at = Utc::now();
                r.update(&transaction)?;
                transaction.commit()?;
            }
            Ok(None) => no_channel_found = true,
            Err(_) => return Err(String::from("Database error.").into()),
        };
    };

    if no_channel_found {
        ctx.send(
            CreateReply::default()
            .content("您的 YouTube 頻道不存在於資料庫，請先在直播聊天室留言以建立您的 YouTube 頻道記錄。")
            .ephemeral(true)
        ).await?;
        return Ok(());
    }

    author
        .dm(
            ctx,
            CreateMessage::new().content("您已成功連結您的帳號至 YouTube 頻道。"),
        )
        .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn unlink(ctx: super::Context<'_>) -> Result<(), ServerError> {
    if check_exist(ctx).await? == false {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "您還沒有將本帳號連結到 YouTube 頻道。\n如要綁定 YouTube 頻道，請使用 {}。",
                    CONFIG.slash_command_strings.link
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let mut id_not_found = false;
    // Here should be the database connection to store the channel id
    let _ = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;

        let _ = match Coin::by_discord(ctx.author().id.get().to_string(), &transaction) {
            Ok(Some(mut r)) => {
                r.discord_id = 0;
                r.updated_at = Utc::now();
                r.update(&transaction)?;
                transaction.commit()?;
            }
            Ok(None) => id_not_found = true,
            Err(_) => return Err(String::from("Account not yet linked").into()),
        };
    };

    if id_not_found {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "您還沒有將本帳號連結到 YouTube 頻道。\n如要綁定 YouTube 頻道，請使用 {}。",
                    CONFIG.slash_command_strings.link
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    ctx.reply("您已成功解除連結您的 YouTube 頻道。").await?;
    Ok(())
}
