use crate::{
    database::{self, video::Video},
    error::ServerError,
    webpage::auth,
};
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, post};
use futures_util::TryStreamExt as _;
use serde_json::{Value, json};

#[post("/api/video/upload-json")]
pub async fn handler(mut payload: Multipart) -> Result<impl Responder, ServerError> {
    let mut token: Option<String> = None;
    let mut file_bytes: Option<Vec<u8>> = None;

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
            data.extend_from_slice(&chunk);
        }

        match field_name.as_str() {
            "token" => match String::from_utf8(data) {
                Ok(value) => token = Some(value.trim().to_string()),
                Err(_) => token = Some(String::new()),
            },
            "file" => file_bytes = Some(data),
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

    let file_bytes = match file_bytes {
        Some(bytes) => bytes,
        None => return Ok(HttpResponse::BadRequest().body("missing file")),
    };

    let raw_json: Value = match serde_json::from_slice(&file_bytes) {
        Ok(value) => value,
        Err(_) => return Ok(HttpResponse::BadRequest().body("invalid json")),
    };

    let videos: Vec<Video> = match serde_json::from_value(raw_json) {
        Ok(videos) => videos,
        Err(_) => return Ok(HttpResponse::BadRequest().body("invalid video payload")),
    };

    let mut inserted = 0usize;
    let mut skipped = 0usize;

    for mut video in videos {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        match video.insert(&transaction) {
            Ok(_) => inserted += 1,
            Err(err) => {
                log::warn!("failed to insert video {}: {}", video.link, err);
                skipped += 1;
            }
        }
        transaction.commit()?;
    }

    Ok(HttpResponse::Ok().json(json!({
        "inserted": inserted,
        "skipped": skipped,
    })))
}
