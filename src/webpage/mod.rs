pub mod auth;
pub mod ping;
pub mod wheel;

use crate::error::ServerError;
use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer, Responder};

pub async fn index() -> Result<impl Responder, ServerError> {
    NamedFile::open_async("dist/index.html")
        .await
        .map_err(|e| e.into())
}

pub fn init() -> Result<(), ServerError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            HttpServer::new(move || {
                App::new()
                    .service(ping::handler)
                    .service(auth::login::login_handler)
                    .service(auth::login::logout_logging)
                    .service(auth::tick::handler)
                    .service(wheel::create::handler)
                    .service(wheel::update::handler)
                    .service(Files::new("/", "dist/").index_file("index.html"))
                    .default_service(web::to(index))
            })
            .bind(("0.0.0.0", 8080))?
            .run()
            .await
        })?;

    Ok(())
}
