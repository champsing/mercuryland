mod chat;
mod video;

use crate::{config::CONFIG, error::ServerError};
use google_youtube3::{
    api::Video, common::Connector, hyper_rustls, hyper_util, yup_oauth2, YouTube,
};
use regex::Regex;
use std::{thread, time::Duration};
use video as h;

pub async fn run() -> Result<(), ServerError> {
    const CHANNEL_ID: &str = "UCHir2DYN4kcH-MpPIT6sXTQ"; // oreki channel id

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        CONFIG.youtube.clone(),
        yup_oauth2::InstalledFlowReturnMethod::HTTPPortRedirect(CONFIG.oauth_redirect_port.clone()),
    )
    .persist_tokens_to_disk("data/youtube.conf")
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

    loop {
        if let Some(id) = get_broadcast_id(&api, CHANNEL_ID).await? {
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
