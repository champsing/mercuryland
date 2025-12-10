mod chat;
mod video;

use crate::{config::CONFIG, database::config::Config, discord, error::ServerError};
use actix_web::cookie::time::{UtcOffset, format_description};
use google_youtube3::{
    YouTube,
    api::Video,
    common::Connector,
    hyper_rustls, hyper_util,
    yup_oauth2::{
        self,
        authenticator_delegate::{DeviceAuthResponse, DeviceFlowDelegate},
    },
};
use regex::Regex;
use serenity::all::CreateMessage;
use std::{fs::OpenOptions, future::Future, pin::Pin, thread, time::Duration};
use video as h;

pub struct FlowDelegateForDiscord(pub discord::Receiver);
impl DeviceFlowDelegate for FlowDelegateForDiscord {
    fn present_user_code<'a>(
        &'a self,
        device_auth_resp: &'a DeviceAuthResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(present_user_code(device_auth_resp, self.0))
    }
}

pub async fn present_user_code(device_auth_resp: &DeviceAuthResponse, recv: discord::Receiver) {
    let printable_time = match UtcOffset::current_local_offset() {
        Ok(offset) => device_auth_resp.expires_at.to_offset(offset),
        Err(_) => device_auth_resp.expires_at, // Fallback to printing in UTC
    };

    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] UTC[offset_hour \
              sign:mandatory]",
    );

    let formatted_time = match format {
        Ok(format) => printable_time.format(&format),
        Err(_) => Ok(printable_time.to_string()),
    };

    match recv
        .message(CreateMessage::new().content(&format!(
            "請在 {} 輸入以下代碼以授予本應用程式權限：",
            device_auth_resp.verification_uri
        )))
        .await
    {
        Err(err) => log::error!("{}", err),
        _ => (),
    }

    match recv
        .message(CreateMessage::new().content(&format!("> {}", device_auth_resp.user_code)))
        .await
    {
        Err(err) => log::error!("{}", err),
        _ => (),
    }

    match recv
        .message(CreateMessage::new().content(&format!(
            "除非您已完成驗證或拒絕授權本應用程式，否則請勿關閉驗證視窗。\n驗證碼將在 {} 後失效。",
            formatted_time.unwrap()
        )))
        .await
    {
        Err(err) => log::error!("{}", err),
        _ => (),
    }
}

pub async fn run() -> Result<(), ServerError> {
    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;

    let query_youtube_id = match Config::YoutubeChannelId.get(&transaction) {
        // 處理 Config::YoutubeChannelId.get 自身的 Result 錯誤
        Ok(channel_id_option) => channel_id_option.expect("No Oreki channel found."),
        Err(_) => {
            return Err(ServerError::Internal(String::from(
                "Fetch Oreki channel id failed.",
            )));
        }
    };

    transaction.commit()?;

    let channel_id: &str = &query_youtube_id.as_str(); // 將 String 轉為 &str

    if OpenOptions::new()
        .read(true)
        .open("data/youtube.secret")
        .is_err()
    {
        discord::Receiver::UserId(CONFIG.discord.admin[0]).message(CreateMessage::new().content(&format!("您正在執行的操作是授權我們存取您的 Google 帳號以讀取您帳號旗下的 YouTube 頻道，用於讀取惡靈直播聊天室的訊息。"))).await?;
    }

    let auth =
        yup_oauth2::DeviceFlowAuthenticator::builder(crate::config::CFG_YOUTUBE_TOKEN.clone())
            .persist_tokens_to_disk("data/youtube.secret")
            .flow_delegate(Box::new(FlowDelegateForDiscord(discord::Receiver::UserId(
                CONFIG.discord.admin[0],
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
    let _ = api
        .channels()
        .list(&vec!["snippet".into()])
        .mine(true)
        .doit()
        .await?;

    println!("YouTube authentication complete");

    loop {
        if let Some(id) = get_broadcast_id(&api, channel_id).await? {
            if let Some(video) = video_from_id(&api, &id).await? {
                h::chat::handle(&api, &video).await?;
            } else {
                log::error!("cannot find broadcast {}", id);
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}

/**
 * return None if no broadcast available
 */
async fn get_broadcast_id<C>(
    _: &YouTube<C>,
    channel: impl Into<String>,
) -> Result<Option<String>, ServerError> {
    let response = reqwest::get(format!(
        "https://www.youtube.com/channel/{}/live",
        channel.into()
    ))
    .await?;
    let text = response.text().await?;

    if text.matches("\"isLive\":true").count() >= 2 {
        let re = Regex::new(r#"video_id=([_0-9a-zA-Z]*)"}"#).unwrap();
        if let Some(captures) = re.captures(&text) {
            let id = &captures[1];
            return Ok(Some(id.into()));
        }
        log::error!("broadcast is ready but no id found");
    }

    Ok(None)
}

async fn video_from_id<C>(api: &YouTube<C>, id: &String) -> Result<Option<Video>, ServerError>
where
    C: Connector,
{
    let part = vec!["liveStreamingDetails".into()];
    let (_, res) = api.videos().list(&part).add_id(id).doit().await?;

    if let Some(videos) = res.items {
        if let Some(video) = videos.into_iter().next() {
            // select the first broadcast available
            return Ok(Some(video));
        }
    }

    return Ok(None);
}
