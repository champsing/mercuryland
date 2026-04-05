use mercury_land::{discord, error::ServerError, webpage, youtube};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), ServerError> {
    env_logger::init();
    mercury_land::database::init()?;

    println!("======== server starting! ========");

    let shutdown = Arc::new(AtomicBool::new(false));

    // 註冊 Ctrl-C handler
    {
        let shutdown = shutdown.clone();
        ctrlc::set_handler(move || {
            println!("\n======== 接收到停機訊號，正在關閉... ========");
            shutdown.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    let mut handles = vec![];

    macro_rules! spawn_service {
        ($name:literal, $shutdown:expr, $run:expr) => {{
            let shutdown = $shutdown.clone();
            thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect(concat!("Failed to build runtime for ", $name));

                loop {
                    if shutdown.load(Ordering::SeqCst) {
                        log::info!("{} shutting down", $name);
                        break;
                    }

                    let res = rt.block_on($run);

                    if shutdown.load(Ordering::SeqCst) {
                        log::info!("{} shutting down", $name);
                        break;
                    }

                    if let Err(err) = res {
                        log::error!("restarting {}, failed: {:?}", $name, err);

                        // 冷卻期間每 100ms 檢查一次停機旗標
                        for _ in 0..600 {
                            if shutdown.load(Ordering::SeqCst) {
                                log::info!("{} shutting down during cooldown", $name);
                                return;
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                    }
                }
            })
        }};
    }

    handles.push(spawn_service!("webpage", shutdown, webpage::run()));
    handles.push(spawn_service!("discord", shutdown, discord::run()));
    handles.push(spawn_service!("youtube", shutdown, youtube::run()));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("======== server stopped safely ========");
    Ok(())
}
