pub use crate::database::coin::Coin;
use crate::{database::get_connection, error::ServerError};
use chrono::{DateTime, TimeDelta, Utc};
use std::{cmp::min, collections::HashMap};

pub struct CoinConfig;

impl CoinConfig {
    /**
     * the coin earned for each message sent, unless otherwise
     * specified. Guarded by `daily_quota`.
     */
    fn coin_per_message(&self, is_sponsor: bool) -> i64 {
        if is_sponsor {
            2
        } else {
            1
        }
    }

    /**
     * the coin earned for the first message sent for each day.
     * Guarded by `daily_quota`.
     */
    fn first_message_coin(&self, is_sponsor: bool) -> i64 {
        if is_sponsor {
            20
        } else {
            10
        }
    }

    /**
     * the maximum coin earned for a day.
     */
    fn daily_quota(&self, is_sponsor: bool) -> i64 {
        if is_sponsor {
            100
        } else {
            50
        }
    }
}

pub struct CoinManager {
    config: CoinConfig,
    refresh: DateTime<Utc>,
    quota: HashMap<String, i64>,
    spam: HashMap<String, DateTime<Utc>>,
}

impl CoinManager {
    pub fn new() -> Self {
        Self {
            config: CoinConfig,
            refresh: Utc::now(),
            quota: HashMap::new(),
            spam: HashMap::new(),
        }
    }

    pub fn youtube_engagement(
        &mut self,
        author: &String,
        is_sponsor: bool,
        event_type: &String,
        published_at: DateTime<Utc>,
    ) -> Result<(), ServerError> {
        let coin = if is_text_message(event_type) && !self.is_spam(author, published_at) {
            self.reset_quota(published_at);

            // initial coin based on if user exists
            let coin = if self.quota.contains_key(author) {
                self.config.coin_per_message(is_sponsor)
            } else {
                self.config.first_message_coin(is_sponsor)
            };
            // apply quota
            self.apply_quota(coin, author, is_sponsor)
        } else {
            0
        };

        if coin != 0 {
            let mut connection = get_connection()?;
            let transaction = connection.transaction()?;

            let mut table = Coin::get_or_create(author, &transaction)?;
            table.coin += coin;
            table.updated_at = published_at;
            table.update(&transaction)?;

            transaction.commit()?;
        }

        Ok(())
    }

    fn is_spam(&mut self, author: &String, now: DateTime<Utc>) -> bool {
        let is_spam =
            self.spam.contains_key(author) && now < self.spam[author] + TimeDelta::seconds(30);

        if !is_spam {
            self.spam.insert(author.clone(), now);
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
