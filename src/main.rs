use mercury_land::{discord, error::ServerError, webpage, youtube};
use std::thread;
use stderrlog;

fn main() -> Result<(), ServerError> {
    stderrlog::new().module(module_path!()).init().unwrap();

    mercury_land::database::init()?;

    let discord = thread::spawn(|| -> Result<(), ServerError> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(async { discord::run().await })?;

        Ok(())
    });

    let webpage = thread::spawn(|| -> Result<(), ServerError> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async { webpage::run().await })?;

        Ok(())
    });

    let youtube = thread::spawn(|| -> Result<(), ServerError> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async { youtube::run().await })?;

        Ok(())
    });

    webpage.join().unwrap()?;
    discord.join().unwrap()?;
    youtube.join().unwrap()?;

    Ok(())
}
