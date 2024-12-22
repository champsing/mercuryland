pub mod auth;
pub mod ping;
pub mod wheel;

use crate::error::ServerError;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
