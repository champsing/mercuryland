mod chat;
mod video;

use crate::{config::CONFIG, discord, error::ServerError};
use actix_web::cookie::time::{format_description, UtcOffset};
use google_youtube3::{
    api::Video,
    common::Connector,
    hyper_rustls, hyper_util,
    yup_oauth2::{
        self,
        authenticator_delegate::{DeviceAuthResponse, DeviceFlowDelegate},
    },
    YouTube,
};
use regex::Regex;
use serenity::all::ChannelId;
use std::{
    future::Future,
    pin::Pin,
    thread::{self},
    time::Duration,
};
use video as h;

pub struct FlowDelegateForDiscord(pub ChannelId);
impl DeviceFlowDelegate for FlowDelegateForDiscord {
    fn present_user_code<'a>(
        &'a self,
        device_auth_resp: &'a DeviceAuthResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(present_user_code(device_auth_resp, self.0))
    }
}

pub async fn present_user_code(device_auth_resp: &DeviceAuthResponse, channel_id: ChannelId) {
    discord::send_text(
        channel_id,
        &format!(
            "請在 {} 輸入 {} 以授予本應用程式權限。",
            device_auth_resp.verification_uri, device_auth_resp.user_code
        ),
    )
    .await
    .unwrap();

    discord::send_text(
        channel_id,
        &format!("除非您已完成驗證或拒絕授權本應用程式，否則請勿關閉驗證視窗。"),
    )
    .await
    .unwrap();

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

    discord::send_text(
        channel_id,
        &format!("驗證碼將在 {} 後失效。", formatted_time.unwrap()),
    )
    .await
    .unwrap();
}

pub async fn run() -> Result<(), ServerError> {
    let channel_id: &str = CONFIG.youtube_channel_id.as_str(); // oreki channel id

    let auth = yup_oauth2::DeviceFlowAuthenticator::builder(CONFIG.yt_chat_viewer.clone())
        .persist_tokens_to_disk("data/youtube_chat_viewer.conf")
        .flow_delegate(Box::new(FlowDelegateForDiscord(
            CONFIG.discord_channel_id.admin.into(),
        )))
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
