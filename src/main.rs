use actix_files::Files;
use actix_web::{web, App, HttpServer};
use mercury_land::webpage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if let Err(error) = mercury_land::init() {
        panic!("Fail to initialize the server with error: {}", error)
    }

    HttpServer::new(move || {
        App::new()
            .service(webpage::ping::handler)
            .service(webpage::auth::login::login_handler)
            .service(webpage::auth::login::logout_logging)
            .service(webpage::auth::tick::handler)
            .service(webpage::wheel::create::handler)
            .service(webpage::wheel::update::handler)
            .service(Files::new("/", "dist/").index_file("index.html"))
            .default_service(web::to(webpage::index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
