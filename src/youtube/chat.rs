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

pub mod logging {
    use super::*;

    #[allow(dead_code)]
    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        unwrap_or_return!(author, chat);
        unwrap_or_return!(message, chat);

        println!("{}: {}", author, message);

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
        unwrap_or_return!(author, chat);
        unwrap_or_return!(is_sponsor, chat);

        let mut manager = CONTEXT.lock().await;
        manager.chat(author, is_sponsor, event_type, published_at)?;

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
        unwrap_or_return!(author, chat);
        unwrap_or_return!(message, chat);

        if event_type != "textMessageEvent" {
            return Ok(());
        }

        if message.starts_with("/booster ") {
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
                .buy_booster(author, level, &content, published_at)
                .await?;
        }

        Ok(())
    }
}
