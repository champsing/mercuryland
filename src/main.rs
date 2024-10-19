use actix_files::Files;
use actix_web::{App, HttpServer};
use mercury_land::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::ping::handler)
            .service(web::auth::login::handler)
            .service(web::auth::tick::handler)
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
