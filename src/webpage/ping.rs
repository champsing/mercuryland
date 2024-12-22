use actix_web::{get, HttpResponse, Responder};

#[get("/api/ping")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
