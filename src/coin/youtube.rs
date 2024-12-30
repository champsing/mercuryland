use super::config::CoinConfig;
pub use crate::database::coin::Coin;
use crate::{database::get_connection, error::ServerError};
use chrono::{DateTime, TimeDelta, Utc};
use std::{cmp::min, collections::HashMap};

pub struct CoinChatManager {
    config: CoinConfig,
    refresh: DateTime<Utc>,
    quota: HashMap<String, i64>,
    spam: HashMap<String, DateTime<Utc>>,
}

impl CoinChatManager {
    pub fn new() -> Self {
        Self {
            config: CoinConfig,
            refresh: Utc::now(),
            quota: HashMap::new(),
            spam: HashMap::new(),
        }
    }

    pub fn chat(
        &mut self,
        author_id: &String,
        author_name: &String,
        is_sponsor: bool,
        event_type: &String,
        now: DateTime<Utc>,
    ) -> Result<(), ServerError> {
        let coin = if is_text_message(event_type) && !self.is_spam(author_id, now) {
            self.reset_quota(now);

            // initial coin based on if user exists
            let coin = if self.quota.contains_key(author_id) {
                self.config.coin_per_message(is_sponsor)
            } else {
                self.config.first_message_coin(is_sponsor)
            };
            // apply quota
            self.apply_quota(coin, author_id, is_sponsor)
        } else {
            0
        };

        if coin != 0 {
            println!("[+] user {} receive ${}", author_id, coin);

            let mut connection = get_connection()?;
            let transaction = connection.transaction()?;

            let mut record = Coin::get_or_create(author_id, author_name, &transaction)?;
            record.coin += coin;
            record.updated_at = now;
            record.update(&transaction)?;

            transaction.commit()?;
        }

        Ok(())
    }

    fn is_spam(&mut self, author_id: &String, now: DateTime<Utc>) -> bool {
        let is_spam = self.spam.contains_key(author_id)
            && now < self.spam[author_id] + TimeDelta::seconds(30);

        if !is_spam {
            self.spam.insert(author_id.clone(), now);
        }

        is_spam
    }

    fn reset_quota(&mut self, now: DateTime<Utc>) {
        if now > self.refresh + TimeDelta::days(1) {
            self.refresh += TimeDelta::days(1);
            self.quota.clear();
        }
    }

    fn apply_quota(&mut self, coin: i64, author: &String, is_sponsor: bool) -> i64 {
        let remaining = self
            .quota
            .entry(author.clone())
            .or_insert(self.config.daily_quota(is_sponsor));

        let coin = min(coin, *remaining);
        *remaining -= coin;
        coin
    }
}

fn is_text_message(event_type: &String) -> bool {
    event_type == "textMessageEvent"
}
