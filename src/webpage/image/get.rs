use crate::{
    database::{self, image::Image},
    error::ServerError,
};
use actix_web::{HttpResponse, Responder, get, web};
use uuid::Uuid;

#[get("/api/image/get/{name}")]
pub async fn handler(path: web::Path<String>) -> Result<impl Responder, ServerError> {
    let name_str = path.into_inner();
    let name = match Uuid::parse_str(&name_str) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid UUID")),
    };

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;
    let image = match Image::by_name(&name, &transaction)? {
        Some(img) => img,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok()
        .insert_header(("Cache-Control", "public, max-age=31536000, immutable"))
        .insert_header(("ETag", format!(r#""{}""#, name)))
        .content_type(image.mime)
        .body(image.data))
}
