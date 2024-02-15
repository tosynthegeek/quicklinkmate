use crate::shortner;
use crate::store;

use actix_web::{App, HttpServer};
use serde::{Deserialize, Serialize};
use validator::Validate;

struct UrlCreationRequest {
    #[serde(rename = "long_url")]
    #[validate(length(min = 1))]
    long_url: String,
}
pub async fn index() -> web::Json<Content> {
    let full_short_url = format!("{}{}", host, short_url);
}

pub async fn create_short_url() {
    let host = "http://localhost:3030/";
    let dt = Local::now();
    let original_url = UrlCreationRequest.long_url;
    let short_url = shortner::generate_short_url(original_url, dt);
    store::save_url_mapping(short_url, original_url);

    HttpResponse::Ok().json(json!({
        "message": "short url created successfully",
        "short_url": host + short_url,
    }));

    short_url
}

pub async fn handle_short_url_redirect(info: web::Path<String>) -> HttpResponse {
    let short_url = info.into_inner();
    let initial_url = store::retrieve_initial_url(&short_url);
    HttpResponse::Found()
        .header("Location", initial_url)
        .finish()
}
