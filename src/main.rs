use mercury_land::{discord, error::ServerError, webpage, youtube};
use std::thread;

fn main() -> Result<(), ServerError> {
    env_logger::init();
    mercury_land::database::init()?;

    let webpage = thread::spawn(|| loop {
        let res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|err| ServerError::from(err))
            .and_then(|rt| rt.block_on(async { webpage::run().await }));

        if let Some(err) = res.err() {
            log::error!("restarting, webpage failed: {}", err)
        }
    });

    let discord = thread::spawn(|| loop {
        let res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|err| ServerError::from(err))
            .and_then(|rt| rt.block_on(async { discord::run().await }));

        if let Some(err) = res.err() {
            log::error!("restarting, discord bot failed: {}", err)
        }
    });

    let youtube = thread::spawn(|| loop {
        let res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|err| ServerError::from(err))
            .and_then(|rt| rt.block_on(async { youtube::run().await }));

        if let Some(err) = res.err() {
            log::error!("restarting, youtube bot failed: {}", err)
        }
    });

    webpage.join().unwrap();
    discord.join().unwrap();
    youtube.join().unwrap();

    Ok(())
}
