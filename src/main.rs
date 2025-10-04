use mercury_land::{discord, error::ServerError, webpage, youtube};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), ServerError> {
    env_logger::init();
    mercury_land::database::init()?;

    println!("======== server starting! ========");

    let mut handles = vec![];

    handles.push(thread::spawn(|| {
        loop {
            let res = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .map_err(|err| ServerError::from(err))
                .and_then(|rt| rt.block_on(async { webpage::run().await }));

            if let Some(err) = res.err() {
                log::error!("restarting, webpage failed: {:?}", err);
                thread::sleep(Duration::from_secs(60));
            }
        }
    }));

    handles.push(thread::spawn(|| {
        loop {
            let res = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .map_err(|err| ServerError::from(err))
                .and_then(|rt| rt.block_on(async { discord::run().await }));

            if let Some(err) = res.err() {
                log::error!("restarting, discord bot failed: {:?}", err);
                thread::sleep(Duration::from_secs(60));
            }
        }
    }));

    handles.push(thread::spawn(|| {
        loop {
            let res = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .map_err(|err| ServerError::from(err))
                .and_then(|rt| rt.block_on(async { youtube::run().await }));

            if let Some(err) = res.err() {
                log::error!("restarting, youtube bot failed: {:?}", err);
                thread::sleep(Duration::from_secs(60));
            }
        }
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
