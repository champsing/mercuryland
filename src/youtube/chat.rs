use crate::error::ServerError;
use chrono::{DateTime, Utc};
use google_youtube3::{api::LiveChatMessage, common::Connector, YouTube};

macro_rules! unwrap_or_return {
    ($name:ident, $source:ident) => {
        let $name = match $name($source) {
            Some(x) => x,
            None => return Ok(()),
        };
    };
}

fn event_type(chat: &LiveChatMessage) -> Option<&String> {
    if let Some(content) = chat.snippet.as_ref() {
        if let Some(type_) = content.type_.as_ref() {
            return Some(type_);
        }
    }

    None
}

fn published_at(chat: &LiveChatMessage) -> Option<DateTime<Utc>> {
    if let Some(content) = chat.snippet.as_ref() {
        if let Some(message) = content.published_at.as_ref() {
            return Some(*message);
        }
    }

    None
}

fn message(chat: &LiveChatMessage) -> Option<&String> {
    if let Some(content) = chat.snippet.as_ref() {
        if let Some(message) = content.display_message.as_ref() {
            return Some(message);
        }
    }

    None
}

fn author_id(chat: &LiveChatMessage) -> Option<&String> {
    if let Some(content) = chat.author_details.as_ref() {
        if let Some(id) = content.channel_id.as_ref() {
            return Some(id);
        }
    }

    None
}

fn author_name(chat: &LiveChatMessage) -> Option<&String> {
    if let Some(content) = chat.author_details.as_ref() {
        if let Some(name) = content.display_name.as_ref() {
            return Some(name);
        }
    }

    None
}

fn is_sponsor(chat: &LiveChatMessage) -> Option<bool> {
    if let Some(content) = chat.author_details.as_ref() {
        if let Some(is_sponsor) = content.is_chat_sponsor.as_ref() {
            return Some(*is_sponsor);
        }
    }

    None
}

pub mod logging {
    use super::*;

    #[allow(dead_code)]
    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        unwrap_or_return!(author_id, chat);
        unwrap_or_return!(message, chat);

        println!("{}: {}", author_id, message);

        Ok(())
    }
}

pub mod coin {
    use super::*;
    use crate::coin::youtube::CoinChatManager;
    use serenity::futures::lock::Mutex;
    use std::sync::LazyLock;

    static CONTEXT: LazyLock<Mutex<CoinChatManager>> =
        LazyLock::new(|| Mutex::new(CoinChatManager::new()));

    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        unwrap_or_return!(event_type, chat);
        unwrap_or_return!(published_at, chat);
        unwrap_or_return!(author_id, chat);
        unwrap_or_return!(author_name, chat);
        unwrap_or_return!(is_sponsor, chat);

        let mut manager = CONTEXT.lock().await;
        manager.chat(author_id, author_name, is_sponsor, event_type, published_at)?;

        Ok(())
    }
}

pub mod command {
    use super::*;
    use crate::coin::command::CoinCommandManager;
    use itertools::Itertools;
    use serenity::futures::lock::Mutex;
    use std::sync::LazyLock;

    static CONTEXT: LazyLock<Mutex<CoinCommandManager>> =
        LazyLock::new(|| Mutex::new(CoinCommandManager::new()));

    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        unwrap_or_return!(event_type, chat);
        unwrap_or_return!(published_at, chat);
        unwrap_or_return!(author_id, chat);
        unwrap_or_return!(message, chat);

        if event_type != "textMessageEvent" {
            return Ok(());
        }

        if message.starts_with("/purchase booster ") {
            let mut split = message.split_ascii_whitespace();
            let _ = split.next(); // the first one is command, ignore.

            let level = split.next().unwrap_or("").parse().unwrap_or(0);
            if level == 0 {
                log::warn!("{}: incorrect level", message);
                return Ok(());
            }

            let content = split.join(" ");

            let manager = CONTEXT.lock().await;
            manager
                .buy_booster(author_id, level, &content, published_at)
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use google_youtube3::api::{
        LiveChatMessage, LiveChatMessageAuthorDetails, LiveChatMessageSnippet,
    };

    fn sample_message() -> LiveChatMessage {
        LiveChatMessage {
            snippet: Some(LiveChatMessageSnippet {
                type_: Some("textMessageEvent".to_string()),
                published_at: Some(
                    Utc.timestamp_opt(1_704_164_245, 0)
                        .single()
                        .expect("valid timestamp"),
                ),
                display_message: Some("/purchase booster 3 Hello".to_string()),
                ..Default::default()
            }),
            author_details: Some(LiveChatMessageAuthorDetails {
                channel_id: Some("channel-id".to_string()),
                display_name: Some("Display".to_string()),
                is_chat_sponsor: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn extracts_event_type() {
        let msg = sample_message();
        assert_eq!(event_type(&msg).map(|s| s.as_str()), Some("textMessageEvent"));
    }

    #[test]
    fn extracts_published_timestamp() {
        let msg = sample_message();
        let expected = Utc.timestamp_opt(1_704_164_245, 0)
            .single()
            .expect("valid timestamp");
        assert_eq!(published_at(&msg), Some(expected));
    }

    #[test]
    fn extracts_message_text() {
        let msg = sample_message();
        assert_eq!(message(&msg).map(|s| s.as_str()), Some("/purchase booster 3 Hello"));
    }

    #[test]
    fn extracts_author_fields() {
        let msg = sample_message();
        assert_eq!(author_id(&msg).map(|s| s.as_str()), Some("channel-id"));
        assert_eq!(author_name(&msg).map(|s| s.as_str()), Some("Display"));
    }

    #[test]
    fn extracts_membership_flag() {
        let msg = sample_message();
        assert_eq!(is_sponsor(&msg), Some(true));
    }

    #[test]
    fn missing_fields_return_none() {
        let msg = LiveChatMessage::default();
        assert!(event_type(&msg).is_none());
        assert!(published_at(&msg).is_none());
        assert!(message(&msg).is_none());
        assert!(author_id(&msg).is_none());
        assert!(author_name(&msg).is_none());
        assert!(is_sponsor(&msg).is_none());
    }
}
