use crate::error::ServerError;
use google_youtube3::YouTube;
use google_youtube3::{api::Video, common::Connector};
use std::{thread, time::Duration};

fn chat_id(video: &Video) -> Option<&String> {
    if let Some(content) = video.live_streaming_details.as_ref() {
        if let Some(chat_id) = content.active_live_chat_id.as_ref() {
            return Some(chat_id);
        }
    }

    None
}

pub mod chat {
    use super::super::chat as h;
    use super::*;

    pub async fn handle<C>(
        api: &YouTube<C>,
        video: &Video,
    ) -> Result<(), ServerError>
    where
        C: Connector,
    {
        if let Some(chat_id) = chat_id(video) {
            let part = vec!["snippet".into(), "authorDetails".into()];

            let mut next_page: Option<String> = None;
            loop {
                let (_, res) = if let Some(token) = next_page {
                    api.live_chat_messages()
                        .list(&chat_id, &part)
                        .page_token(&token)
                        .doit()
                        .await?
                } else {
                    api.live_chat_messages()
                        .list(&chat_id, &part)
                        .doit()
                        .await?
                };

                // get arguments for next request
                next_page = res.next_page_token;
                let polling_ms = res.polling_interval_millis.unwrap_or(0);

                // process messages
                if let Some(messages) = res.items {
                    for message in messages {
                        h::log::run(api, &message).await?;
                        h::coin::run(api, &message).await?;
                    }
                }

                // no more messages, exit
                if let Some(_) = res.offline_at {
                    break;
                }

                thread::sleep(Duration::from_millis(polling_ms as u64));
            }
        }

        Ok(())
    }
}
