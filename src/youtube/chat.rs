use crate::error::ServerError;
use chrono::{DateTime, Utc};
use google_youtube3::{api::LiveChatMessage, common::Connector, YouTube};

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

fn author(chat: &LiveChatMessage) -> Option<&String> {
    if let Some(content) = chat.author_details.as_ref() {
        if let Some(id) = content.channel_id.as_ref() {
            return Some(id);
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

pub mod log {
    use super::*;

    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        if let Some(author) = author(chat) {
            if let Some(message) = message(chat) {
                println!("{}: {}", author, message);
            }
        }

        Ok(())
    }
}

pub mod coin {
    use super::*;
    use crate::coin::CoinManager;
    use serenity::futures::lock::Mutex;
    use std::sync::LazyLock;

    static CONTEXT: LazyLock<Mutex<CoinManager>> = LazyLock::new(|| Mutex::new(CoinManager::new()));

    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        if let Some(event_type) = event_type(chat) {
            if let Some(published_at) = published_at(chat) {
                if let Some(author) = author(chat) {
                    if let Some(is_sponsor) = is_sponsor(chat) {
                        let mut manager = CONTEXT.lock().await;
                        manager.youtube_engagement(author, is_sponsor, event_type, published_at)?;
                    }
                }
            }
        }

        Ok(())
    }
}
