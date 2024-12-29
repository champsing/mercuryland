use crate::error::ServerError;
use google_youtube3::api::{LiveBroadcast, LiveChatMessage, Video};
use google_youtube3::common::Connector;
use google_youtube3::YouTube;
use regex::Regex;
use std::thread;
use std::time::Duration;

pub(super) struct YoutubeBot<C>
where
    C: Connector,
{
    youtube: YouTube<C>,
}

impl<C> YoutubeBot<C>
where
    C: Connector,
{
    pub fn new(youtube: YouTube<C>) -> Self {
        Self { youtube }
    }

    pub async fn start(&self) -> Result<(), ServerError> {
        loop {
            if let Some(id) = self.is_broadcast_ready().await? {
                let broadcast = self
                    .get_video(&id)
                    .await?
                    .ok_or::<ServerError>(format!("cannot find broadcast {}", id).into())?;
                self.handle_broadcast(&broadcast).await?;
            }

            thread::sleep(Duration::from_secs(60));
        }
    }

    async fn is_broadcast_ready(&self) -> Result<Option<String>, ServerError> {
        const CHANNEL_ID: &str = "UCalt_7k09pL6OxW36grt6Ug";
        let response = reqwest::get(format!(
            "https://www.youtube.com/channel/{}/live",
            CHANNEL_ID
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

    async fn get_video(&self, id: &String) -> Result<Option<Video>, ServerError> {
        let part = vec!["liveStreamingDetails".into()];
        let (_, res) = self
            .youtube
            .videos()
            .list(&part)
            .add_id(id)
            .doit()
            .await?;

        if let Some(videos) = res.items {
            if let Some(video) = videos.into_iter().next() {
                // select the first broadcast available
                return Ok(Some(video));
            }
        }

        return Ok(None);
    }

    async fn handle_broadcast(&self, broadcast: &Video) -> Result<(), ServerError> {
        let live_chat_id = broadcast
            .live_streaming_details
            .as_ref()
            .map(|x| x.active_live_chat_id.as_ref())
            .flatten()
            .ok_or::<ServerError>(String::from("no live chat id").into())?;
        let part = vec!["snippet".into(), "authorDetails".into()];

        let mut next_page: Option<String> = None;
        loop {
            let (_, res) = if let Some(token) = next_page {
                self.youtube
                    .live_chat_messages()
                    .list(&live_chat_id, &part)
                    .page_token(&token)
                    .doit()
                    .await?
            } else {
                self.youtube
                    .live_chat_messages()
                    .list(&live_chat_id, &part)
                    .doit()
                    .await?
            };

            // get arguments for next request
            next_page = res.next_page_token;
            let polling_ms = res.polling_interval_millis.unwrap_or(0);

            // process messages
            if let Some(messages) = res.items {
                for message in messages {
                    self.handle_message(&message).await?;
                }
            }

            // no more messages, exit
            if let Some(_) = res.offline_at {
                break;
            }

            thread::sleep(Duration::from_millis(polling_ms as u64));
        }

        Ok(())
    }

    async fn handle_message(&self, message: &LiveChatMessage) -> Result<(), ServerError> {
        println!("-------------- {:?}", message);

        Ok(())
    }
}
