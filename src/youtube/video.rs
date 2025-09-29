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
    use google_youtube3::api::LiveChatMessage;

    use super::super::chat as h;
    use super::*;

    pub async fn handle<C>(api: &YouTube<C>, video: &Video) -> Result<(), ServerError>
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
                // minimum 5 seconds to ensure no overflow
                let polling_ms = res.polling_interval_millis.unwrap_or(0).max(10000) as u64;

                // process messages
                if let Some(chats) = res.items {
                    for chat in chats {
                        if let Err(err) = run_chat(api, &chat).await {
                            log::error!("{:?} for {:?}", err, chat);
                        }
                    }
                }

                // no more messages, exit
                if let Some(_) = res.offline_at {
                    break;
                }

                thread::sleep(Duration::from_millis(polling_ms));
            }
        }

        Ok(())
    }

    async fn run_chat<C>(api: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        // h::log::run(api, &message).await?;
        h::coin::run(api, &chat).await?;
        h::command::run(api, &chat).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use google_youtube3::api::VideoLiveStreamingDetails;

    #[test]
    fn chat_id_returns_value_when_present() {
        let mut video = Video::default();
        video.live_streaming_details = Some(VideoLiveStreamingDetails {
            active_live_chat_id: Some("abc123".to_string()),
            ..Default::default()
        });

        assert_eq!(chat_id(&video).map(|s| s.as_str()), Some("abc123"));
    }

    #[test]
    fn chat_id_returns_none_when_missing() {
        assert!(chat_id(&Video::default()).is_none());
    }
}
