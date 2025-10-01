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
    
    // Add user agent to avoid being blocked
    let client = reqwest::Client::new();
    let html = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;

    let re =
        Regex::new(r#"(?s)<script[^>]*type="application/ld\+json"[^>]*>(.*?)</script>"#).unwrap();

    for caps in re.captures_iter(&html) {
        let raw = caps.get(1).unwrap().as_str().trim();

        println!("Extracted JSON-LD: {}", raw);

        if let Ok(val) = serde_json::from_str::<Value>(raw) {
            println!("Parsed JSON-LD: {:?}", val);

            // Try multiple possible field names for title
            let title = val["title"].as_str()
                .or_else(|| val["name"].as_str())
                .map(|s| s.to_string());
            
            let upload_date = val["uploadDate"].as_str()
                .map(|s| s.to_string());
            
            let duration_iso = val["duration"].as_str()
                .map(|s| s.to_string());

            // Only return if we found at least a title
            if title.is_some() {
                return Ok(HttpResponse::Ok().json(MetadataResponse {
                    title,
                    upload_date,
                    duration: duration_iso,
                }));
            }
        }
    }

    // Fallback: try to extract title from page title tag
    let title_re = Regex::new(r#"<title[^>]*>(.*?)</title>"#).unwrap();
    if let Some(caps) = title_re.captures(&html) {
        let title = caps.get(1).unwrap().as_str().trim();
        // Remove " - YouTube" suffix if present
        let clean_title = title.strip_suffix(" - YouTube").unwrap_or(title);
        
        return Ok(HttpResponse::Ok().json(MetadataResponse {
            title: Some(clean_title.to_string()),
            upload_date: None,
            duration: None,
        }));
    }

    Ok(HttpResponse::BadRequest().body("Failed to extract metadata"))
}
