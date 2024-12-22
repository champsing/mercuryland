use actix_files::Files;
use actix_web::{App, HttpServer};
use mercury_land::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if let Err(error) = mercury_land::init() {
        panic!("Fail to initialize the server with error: {}", error)
    }
    
    HttpServer::new(move || {
        App::new()
            .service(web::ping::handler)
            .service(web::auth::login::login_handler)
            .service(web::auth::login::logout_logging)
            .service(web::auth::tick::handler)
            .service(web::wheel::create::handler)
            .service(web::wheel::update::handler)
            .service(Files::new("/", "dist/").index_file("index.html"))
            .default_service(actix_web::web::to(mercury_land::index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
