use crate::error::ServerError;
use actix_web::{HttpResponse, Responder, post, web};
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MetadataRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct MetadataResponse {
    pub title: Option<String>,
    pub date: Option<String>,
    pub duration: Option<String>,
}

#[post("/api/video/metadata")]
pub async fn handler(req: web::Json<MetadataRequest>) -> Result<impl Responder, ServerError> {
    let url = format!("https://www.youtube.com/watch?v={}", &req.url);
    
    // Simple HTTP request since microdata is available without complex headers
    let client = reqwest::Client::new();
    let html = client
        .get(&url)
        .send()
        .await?
        .text()
        .await?;

    // Extract metadata from HTML meta tags with itemprop (microdata approach)
    println!("Looking for microdata meta tags...");
    
    // Extract title from itemprop="name"
    let title_re = Regex::new(r#"<meta\s+itemprop="name"\s+content="([^"]+)""#).unwrap();
    let title = title_re.captures(&html)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string());
    
    // Extract upload date from itemprop="uploadDate"
    let date_re = Regex::new(r#"<meta\s+itemprop="uploadDate"\s+content="([^"]+)""#).unwrap();
    let date = date_re.captures(&html)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string());
    
    // Extract duration from itemprop="duration"
    let duration_re = Regex::new(r#"<meta\s+itemprop="duration"\s+content="([^"]+)""#).unwrap();
    let duration = duration_re.captures(&html)
        .and_then(|caps| caps.get(1))
        .map(|m| {
            let duration_str = m.as_str();
            // Convert ISO 8601 duration (PT3M12S) to readable format
            if duration_str.starts_with("PT") {
                let mut total_seconds = 0u64;
                let content = &duration_str[2..]; // Remove "PT"
                
                // Parse hours, minutes, seconds
                let mut current_num = String::new();
                for c in content.chars() {
                    if c.is_ascii_digit() {
                        current_num.push(c);
                    } else {
                        if !current_num.is_empty() {
                            let num: u64 = current_num.parse().unwrap_or(0);
                            match c {
                                'H' => total_seconds += num * 3600,
                                'M' => total_seconds += num * 60,
                                'S' => total_seconds += num,
                                _ => {}
                            }
                            current_num.clear();
                        }
                    }
                }
                
                let hours = total_seconds / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;
                
                if hours > 0 {
                    format!("{}:{:02}:{:02}", hours, minutes, seconds)
                } else {
                    format!("{}:{:02}", minutes, seconds)
                }
            } else {
                // Return as-is if not in expected format
                duration_str.to_string()
            }
        });

    println!("Extracted from microdata - title: {:?}, upload_date: {:?}, duration: {:?}", title, date, duration);

    // If we found metadata, return it
    if title.is_some() || date.is_some() || duration.is_some() {
        return Ok(HttpResponse::Ok().json(MetadataResponse {
            title,
            date,
            duration,
        }));
    }

    // Fallback: try to extract title from page title tag
    let title_re = Regex::new(r#"<title[^>]*>(.*?)</title>"#).unwrap();
    if let Some(caps) = title_re.captures(&html) {
        let title = caps.get(1).unwrap().as_str().trim();
        // Remove " - YouTube" suffix if present
        let clean_title = title.strip_suffix(" - YouTube").unwrap_or(title);
        
        return Ok(HttpResponse::Ok().json(MetadataResponse {
            title: Some(clean_title.to_string()),
            date: None,
            duration: None,
        }));
    }

    Ok(HttpResponse::BadRequest().body("Failed to extract metadata"))
}
