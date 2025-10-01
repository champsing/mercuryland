use crate::error::ServerError;
use actix_web::{HttpResponse, Responder, post, web};
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct MetadataRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct MetadataResponse {
    pub title: Option<String>,
    pub upload_date: Option<String>,
    pub duration: Option<String>,
}

#[post("/api/video/metadata")]
pub async fn handler(req: web::Json<MetadataRequest>) -> Result<impl Responder, ServerError> {
    let url = format!("https://www.youtube.com/watch?v={}", &req.url);
    let html = reqwest::get(url).await?.text().await?;

    let re =
        Regex::new(r#"(?s)<script[^>]+type="application/ld\+json"[^>]*>(.*?)</script>"#).unwrap();

    for caps in re.captures_iter(&html) {
        let raw = caps.get(1).unwrap().as_str().trim();

        println!("Extracted JSON-LD: {}", raw);

        if let Ok(val) = serde_json::from_str::<Value>(raw) {
            println!("Parsed JSON-LD: {:?}", val);

            let title = val["name"].as_str();
            let upload_date = val["uploadDate"].as_str();
            let duration_iso = val["duration"].as_str();

            return Ok(HttpResponse::Ok().json(MetadataResponse {
                title: title.map(|s| s.to_string()),
                upload_date: upload_date.map(|s| s.to_string()),
                duration: duration_iso.map(|s| s.to_string()),
            }));
        }
    }

    Ok(HttpResponse::BadRequest().body("Failed to extract metadata"))
}
