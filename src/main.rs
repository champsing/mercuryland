use std::thread;

use mercury_land::{discord, error::ServerError, webpage};

fn main() -> Result<(), ServerError> {
    env_logger::init();

    mercury_land::database::init()?;

    let discord = thread::spawn(|| -> Result<(), ServerError> {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async { discord::run().await })?;

        Ok(())
    });
    
    let webpage = thread::spawn(|| -> Result<(), ServerError> {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async { webpage::run().await })?;

        Ok(())
    });

    discord.join().unwrap()?;
    webpage.join().unwrap()?;

    Ok(())
}
