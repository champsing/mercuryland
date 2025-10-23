use chrono::FixedOffset;
use itertools::Itertools;
use serenity::all::CreateMessage;
use std::iter::once;

use crate::{
    database::{config::Config, penalty::Penalty},
    discord,
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Request {
    password: String,
    penalties: Vec<(String, i32)>,
}

fn state_text(state: i32) -> &'static str {
    match state {
        0 => "未生效",
        1 => "未完成",
        _ => "未知",
    }
}

#[post("/api/wheel/submit")]
pub async fn handler(request: web::Json<Request>) -> Result<impl Responder, ServerError> {
    let mut connection = crate::database::get_connection()?;
    let transaction = connection.transaction()?;

    let wheel_password = if let Some(text) = Config::WheelPassword.get(&transaction)? {
        text
    } else {
        return Ok(HttpResponse::ServiceUnavailable().finish());
    };

    let channel_penalty = if let Some(text) = Config::ChannelPenalty.get(&transaction)?
        && let Ok(channel) = text.parse::<u64>()
    {
        channel
    } else {
        return Ok(HttpResponse::ServiceUnavailable().finish());
    };

    if request.penalties.iter().any(|p| p.1 != 0 && p.1 != 1) {
        // The state must be 未生效 or 未完成
        return Ok(HttpResponse::BadRequest().finish());
    }

    if request.password != wheel_password {
        return Ok(HttpResponse::Forbidden().finish());
    }
    transaction.commit()?;

    let transaction = connection.transaction()?;
    let now = chrono::Utc::now();

    {
        let tz = FixedOffset::east_opt(8 * 3600).unwrap(); // UTC+8
        let date = now.with_timezone(&tz).date_naive();
        for (name, state) in &request.penalties {
            let history = if *state == 0 {
                vec![(0, date)]
            } else if *state == 1 {
                vec![(0, date), (1, date)]
            } else {
                return Ok(HttpResponse::BadRequest().finish());
            };

            let mut penalty = Penalty {
                id: 0, // Placeholder, actual ID will be set by the database
                date: date,
                name: name.clone(),
                state: *state,
                history: history,
                detail: String::new(),
            };
            penalty.insert(&transaction)?;
        }
    }

    {
        let content = request
            .penalties
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}. {} [{}]", i, s.0, state_text(s.1)));

        let message = once(format!("<t:{}:D>", now.timestamp()))
            .chain(content)
            .join("\n");

        discord::Receiver::ChannelId(channel_penalty)
            .message(CreateMessage::new().content(message))
            .await?;
    }

    transaction.commit()?;

    Ok(HttpResponse::Ok().finish())
}
