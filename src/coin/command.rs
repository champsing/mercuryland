use super::config::CoinConfig;
pub use crate::database::coin::Coin;
use crate::{config::CONFIG, database::get_connection, discord, error::ServerError};
use chrono::{DateTime, Utc};
use serenity::all::CreateMessage;

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
        let channel_id: u64 = CONFIG.discord.exchange; // 水星交易所

        // 在一个同步块里处理所有 DB 逻辑，生成好要发送的 message
        let maybe_message = {
            let mut connection = get_connection()?;
            let transaction = connection.transaction()?;

            // 如果用户存在且有足够的 coin，就更新并准备 message
            let msg = if let Some(mut record) = Coin::by_youtube(user, &transaction)? {
                let cost = self.config.booster_cost(level);
                if record.coin >= cost {
                    record.coin -= cost;
                    record.updated_at = now;
                    record.update(&transaction)?;

                    println!(
                        "[-] {} buy a level-{} booster for {}",
                        record.display, level, content
                    );

                    Some(format!(
                        "懲罰加倍: {}x{} (来自 {})，要求退款請在指令區使用 {}。",
                        content, level, record.display, CONFIG.slash_command_strings.refund_new
                    ))
                } else {
                    // coin 不足或其他情况
                    Some(format!(
                        "您的水星幣不足以購買 x{} 的加倍倍率。您可以使用 {} 指令查詢。",
                        level, CONFIG.slash_command_strings.coin
                    ))
                }
            } else {
                None
            };

            transaction.commit()?;
            msg
        };

        // 此处已经不再持有 rusqlite::Transaction，可以安全 .await
        if let Some(content) = maybe_message {
            discord::Receiver::ChannelId(channel_id)
                .message(CreateMessage::new().content(content))
                .await?;
        }
        Ok(())
    }
}
