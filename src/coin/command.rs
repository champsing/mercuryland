use super::config::CoinConfig;
pub use crate::database::coin::Coin;
use crate::{config::CONFIG, database::get_connection, discord, error::ServerError};
use chrono::{DateTime, Utc};
use serde_json::json;

pub struct CoinCommandManager {
    config: CoinConfig,
}

impl CoinCommandManager {
    pub fn new() -> Self {
        Self { config: CoinConfig }
    }

    pub async fn buy_booster(
        &self,
        user: &String,
        level: i64,
        content: &String,
        now: DateTime<Utc>,
    ) -> Result<(), ServerError> {
        let channel_id: u64 = CONFIG.discord_channel_id.public; // 水星交易所

        let mut connection = get_connection()?;
        let transaction = connection.transaction()?;

        if let Some(mut record) = Coin::by_youtube(user, &transaction)? {
            let cost = self.config.booster_cost(level);
            if record.coin >= cost {
                record.coin -= cost;
                record.updated_at = now;
                record.update(&transaction)?;

                println!("[-] {} buy a level-{} booster for {}", user, level, content);

                let content = format!("惩罚加倍: {}x{} (来自{})", content, level, user);
                discord::send_message(channel_id.into(), vec![], &json!({"content": content}))
                    .await?;
            }
        }

        transaction.commit()?;

        Ok(())
    }
}
