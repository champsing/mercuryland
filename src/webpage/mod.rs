pub mod auth;
pub mod leaderboard;
pub mod ping;
pub mod video;
pub mod wheel;

use crate::error::ServerError;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

pub async fn run() -> Result<(), ServerError> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("https://mercuryland.pp.ua")
            .allowed_origin("https://www.mercuryland.pp.ua")
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
            .wrap(cors)
            .service(ping::handler)
            .service(auth::login::login_handler)
            .service(auth::tick::handler)
            .service(wheel::create::handler)
            .service(wheel::update::handler)
            .service(wheel::submit::handler)
            .service(video::list::handler)
            .service(video::upload::handler)
            .service(video::insert::handler)
            .service(video::delete::handler)
            .service(video::update::handler)
            .service(leaderboard::ldb::handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
