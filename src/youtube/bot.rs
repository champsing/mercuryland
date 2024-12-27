use std::thread;
use std::time::Duration;

use crate::error::ServerError;
use google_youtube3::api::{LiveBroadcast, LiveChatMessage};
use google_youtube3::common::Connector;
use google_youtube3::YouTube;
use log::*;

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
            let part = vec!["snippet".into()];
            let (_, res) = self
                .youtube
                .live_broadcasts()
                .list(&part)
                .broadcast_status("active")
                .mine(true)
                .doit()
                .await?;

            if let Some(broadcasts) = res.items {
                if let Some(broadcast) = broadcasts.first() {
                    self.handle_broadcast(broadcast).await?;
                }
            }

            thread::sleep(Duration::from_secs(600));
        }
    }

    async fn handle_broadcast(&self, broadcast: &LiveBroadcast) -> Result<(), ServerError> {
        let live_chat_id = broadcast
            .snippet
            .as_ref()
            .map(|x| x.live_chat_id.as_ref())
            .flatten()
            .ok_or::<ServerError>(String::from("no live chat id").into())?;
        let part = vec!["snippet".into(), "authorDetails".into()];

        info!("Youtube broadcast started");

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
        log::info!("{:?}", message);

        Ok(())
    }
}
