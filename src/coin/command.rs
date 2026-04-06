use super::config::CoinConfig;
pub use crate::database::user::User;
use crate::{
    database::{config::Config, get_connection},
    discord,
    error::ServerError,
};
use chrono::{DateTime, Utc};
use serenity::all::CreateMessage;

pub struct CoinCommandManager {
    pub(crate) config: CoinConfig,
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
        // 1. 建立一個變數來存放 Discord 發送所需的資訊
        let mut notification_payload: Option<(u64, String)> = None;

        // 2. 使用一個獨立的程式碼塊 (Scope) 處理資料庫
        {
            let mut connection = get_connection()?;
            let transaction = connection.transaction()?;

            let channel_coin = if let Some(text) = Config::ChannelCoin.get(&transaction)?
                && let Ok(channel) = text.parse::<u64>()
            {
                channel
            } else {
                return Err(ServerError::Internal(String::from(
                    "Parse ChannelCoin channel id to u64 failed.",
                )));
            };

            if let Some(mut record) = User::by_youtube(user, &transaction)? {
                let cost = self.config.booster_cost(level);
                if record.coin >= cost {
                    record.coin -= cost;
                    record.updated_at = now;
                    record.update(&transaction)?;

                    let due = now.checked_add_days(chrono::Days::new(3)).unwrap();

                    println!("[-] {} buy a level-{} booster for {}", user, level, content);

                    // 在這裡格式化字串，並暫存到 payload 變數中
                    /*
                    # 懲罰加倍 #
                    > ## 「{}」 x **{}** 倍 ##
                    - 來自： {} (`{}`)，結餘 **{}** 水星幣。
                    _此為 YouTube 聊天室指令，須由管理員手動開啟退款單。_
                    -# 如有疑義，或未抽中想領取半價退款，請在 72 小時內（<t:{}:f> 之前）向管理員申請退款。
                     */
                    let message_content = format!(
                        "# 懲罰加倍 #\n> ## 「{}」 x **{}** 倍 ##\n- 來自： {} (`{}`)，結餘 **{}** 水星幣。\n_此為 YouTube 聊天室指令，須由管理員手動開啟退款單._\n-# 如有疑義，或未抽中想領取半價退款，請在 72 小時內（<t:{}:f> 之前）向管理員申請退款。",
                        content,
                        level,
                        record.display,
                        record.youtube,
                        record.coin,
                        due.timestamp()
                    );

                    notification_payload = Some((channel_coin, message_content));
                }
            }

            transaction.commit()?;
            // 當程式執行到此處，這個 Scope 結束，transaction 和 connection 都會被 Drop
        }

        // 3. 在資料庫連線釋放後，才執行非同步的 Discord API 呼叫
        if let Some((channel_id, msg)) = notification_payload {
            discord::Receiver::ChannelId(channel_id)
                .message(CreateMessage::new().content(msg))
                .await?; // 這裡的 await 不再持有資料庫連線，不會報錯
        }

        Ok(())
    }
}
