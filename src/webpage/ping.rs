use actix_web::{HttpResponse, Responder, get};

#[get("/api/ping")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
