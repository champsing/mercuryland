pub mod auth;
pub mod ping;
pub mod wheel;

use crate::error::ServerError;
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer, Responder};

pub async fn index() -> Result<impl Responder, ServerError> {
    NamedFile::open_async("dist/index.html")
        .await
        .map_err(|e| e.into())
}

pub async fn run() -> Result<(), ServerError> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
            .wrap(cors)
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
    .await?;

    Ok(())
}
