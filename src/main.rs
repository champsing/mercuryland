use mercury_land::{discord, error::ServerError, webpage, youtube};
use std::thread;
use stderrlog;

fn main() -> Result<(), ServerError> {
    stderrlog::new().module(module_path!()).init().unwrap();
    mercury_land::database::init()?;

    let webpage = thread::spawn(|| {
        let res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async { webpage::run().await });

        if let Some(err) = res.err() {
            log::error!("discord bot failed: {}", err)
        }
    });

    let discord = thread::spawn(|| {
        let res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(async { discord::run().await });

        if let Some(err) = res.err() {
            log::error!("discord bot failed: {}", err)
        }
    });

    let youtube = thread::spawn(|| {
        let res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async { youtube::run().await });

        if let Some(err) = res.err() {
            log::error!("discord bot failed: {}", err)
        }
    });

    webpage.join().unwrap()?;
    discord.join().unwrap()?;
    youtube.join().unwrap()?;

    Ok(())
}
