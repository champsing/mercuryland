use crate::{
    database::{self, image::Image},
    error::ServerError,
    webpage::auth,
};
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, post};
use futures_util::TryStreamExt as _;
use serde_json::json;
use uuid::Uuid;

#[post("/api/image/upload")]
pub async fn handler(mut payload: Multipart) -> Result<impl Responder, ServerError> {
    let mut token: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut mime: Option<String> = None;

    const MAX_BYTES: usize = 1 * 1024 * 1024; // 1 MB

    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|err| ServerError::Internal(format!("multipart error: {err}")))?
    {
        let field_name = field
            .content_disposition()
            .get_name()
            .map(|name| name.to_string())
            .unwrap_or_default();

        let mut data = Vec::new();
        while let Some(chunk) = field
            .try_next()
            .await
            .map_err(|err| ServerError::Internal(format!("multipart chunk error: {err}")))?
        {
            if data.len() + chunk.len() > MAX_BYTES {
                return Ok(HttpResponse::PayloadTooLarge().body("File too large"));
            }
            data.extend_from_slice(&chunk);
        }

        match field_name.as_str() {
            "token" => match String::from_utf8(data) {
                Ok(value) => token = Some(value.trim().to_string()),
                Err(_) => token = Some(String::new()),
            },
            "file" => {
                file_data = Some(data);
                if let Some(content_type) = field.content_type() {
                    mime = Some(content_type.to_string());
                }
            }
            _ => {}
        }
    }

    let token = match token {
        Some(token) => token,
        None => return Ok(HttpResponse::Forbidden().finish()),
    };

    if !auth::verify(&token) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let data = match file_data {
        Some(data) => data,
        None => return Ok(HttpResponse::BadRequest().body("No file uploaded")),
    };

    if data.is_empty() {
        return Ok(HttpResponse::BadRequest().body("No file uploaded"));
    }

    let mime = match mime {
        Some(m) => {
            if !m.starts_with("image/") {
                return Ok(HttpResponse::UnsupportedMediaType().body("Only image/* allowed"));
            }
            m
        }
        None => "application/octet-stream".to_string(),
    };

    let name = Uuid::new_v5(&Uuid::NAMESPACE_OID, &data);

    let mut connection = database::get_connection()?;
    let transaction = connection.transaction()?;

    // Check if image already exists
    if Image::by_name(&name, &transaction)?.is_some() {
        let url = format!("/api/image/get/{}", name);
        return Ok(HttpResponse::Ok().json(json!({
            "name": name.to_string(),
            "url": url,
            "message": "Image already exists"
        })));
    }

    let mut image = Image {
        id: 0, // will be set by insert
        name,
        data,
        mime,
    };

    image.insert(&transaction)?;
    transaction.commit()?;

    let url = format!("/api/image/get/{}", name);
    Ok(HttpResponse::Ok().json(json!({
        "name": name.to_string(),
        "url": url
    })))
}
