use actix_web::{get, HttpResponse, Responder};

#[get("/ping")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
