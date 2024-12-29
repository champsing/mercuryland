use crate::error::ServerError;
use google_youtube3::{api::LiveChatMessage, common::Connector, YouTube};

fn event_type(chat: &LiveChatMessage) -> Option<&String> {
    if let Some(content) = chat.snippet.as_ref() {
        if let Some(type_) = content.type_.as_ref() {
            return Some(type_);
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

pub mod log {
    use chrono::Utc;
    use std::{fs::OpenOptions, io::Write};

    use super::*;

    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        let message_log = OpenOptions::new().append(true).create(true).open(format!(
            "data/msg_logs/message_log_{}.log",
            Utc::now().date_naive().to_string()
        ));
        if let Some(author) = author(chat) {
            if let Some(message) = message(chat) {
                println!("{}: {}", author, message);

                writeln!(message_log.expect("Can't find log file."), "{}: {}", author, message)
                    .expect("Can't write log.");
            }
        }

        Ok(())
    }
}

pub mod coin {
    use crate::database::{get_connection, user::User};

    use super::*;
    use chrono::{DateTime, TimeDelta, Utc};
    use serenity::futures::lock::Mutex;
    use std::{collections::HashMap, sync::LazyLock};

    const FIRST_MSG: i64 = 10;
    const OTHER_MSG: i64 = 1;
    const DAY_QUOTA: i64 = 50;

    struct Context {
        date: DateTime<Utc>,
        quota: HashMap<String, i64>,
    }

    static CONTEXT: LazyLock<Mutex<Context>> = LazyLock::new(|| {
        Mutex::new(Context {
            date: Utc::now(),
            quota: HashMap::new(),
        })
    });

    pub async fn run<C>(_: &YouTube<C>, chat: &LiveChatMessage) -> Result<(), ServerError>
    where
        C: Connector,
    {
        if let Some(author) = author(chat) {
            if event_type(chat) == Some(&String::from("textMessageEvent")) {
                let coin;
                let now = Utc::now();
                {
                    let mut ctx = CONTEXT.lock().await;

                    // reset all quota after 1 day
                    if now > ctx.date + TimeDelta::days(1) {
                        ctx.date += TimeDelta::days(1);
                        ctx.quota.clear();
                    }

                    if !ctx.quota.contains_key(author) {
                        // first message in a day
                        coin = FIRST_MSG;
                        ctx.quota.insert(author.clone(), DAY_QUOTA - FIRST_MSG);
                    } else {
                        coin = OTHER_MSG;
                        *ctx.quota.get_mut(author).unwrap() -= OTHER_MSG;
                    }
                }
                let mut connection = get_connection()?;
                let transaction = connection.transaction()?;
                let mut user = User::get_or_create(author, &transaction)?;
                user.coin += coin;
                user.updated_at = now;
                user.update(&transaction)?;
                transaction.commit()?;
            }
        }

        Ok(())
    }
}
