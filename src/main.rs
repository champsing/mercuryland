use mercury_land::{discord, error::ServerError, webpage, youtube};
use tokio::signal;

#[tokio::main] // 使用 tokio 提供的 macro
async fn main() -> Result<(), ServerError> {
    env_logger::init();
    mercury_land::database::init()?;

    println!("======== server starting! ========");

    // 使用 tokio::spawn 建立並行任務
    let webpage_task = tokio::spawn(async {
        loop {
            match webpage::run().await {
                Ok(()) => break, // 收到停機訊號，優雅退出
                Err(err) => {
                    log::error!("restarting, webpage failed: {:?}", err);
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                }
            }
        }
    });

    let discord_task = tokio::spawn(async {
        loop {
            match discord::run().await {
                Ok(()) => break,
                Err(err) => {
                    log::error!("restarting, discord bot failed: {:?}", err);
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                }
            }
        }
    });

    let youtube_task = tokio::spawn(async {
        loop {
            match youtube::run().await {
                Ok(()) => break,
                Err(err) => {
                    log::error!("restarting, youtube bot failed: {:?}", err);
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                }
            }
        }
    });

    // 等待所有任務完成，或監聽主進程的 Ctrl+C
    tokio::select! {
        _ = webpage_task => log::info!("Webpage task finished"),
        _ = discord_task => log::info!("Discord task finished"),
        _ = youtube_task => log::info!("Youtube task finished"),
        _ = signal::ctrl_c() => {
            println!("\n======== Received Ctrl+C, shutting down... ========");
        }
    }

    println!("======== server stopped! ========");
    Ok(())
}
