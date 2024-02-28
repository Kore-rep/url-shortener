use crate::constants::{APPLICATION_JSON, REDIRECT_SERVER_URL};
use actix_web::{post, web::Json, HttpResponse};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
pub struct ShortenedUrlData {
    pub key: String,
    pub short_url: String,
    pub long_url: String,
}

impl ShortenedUrlData {
    pub fn new(url: &String) -> Self {
        let url_hash = hash_url::<String>(url);
        Self {
            key: url_hash.to_string(),
            short_url: [REDIRECT_SERVER_URL.to_string(), url_hash].join("/"),
            long_url: url.to_string(),
        }
    }
}

fn hash_url<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let hash = s.finish().to_string();
    hash[0..6].to_string()
}

#[post("/url/shorten")]
pub async fn shorten(shorten_req: Json<ShortenRequest>) -> HttpResponse {
    // TODO Check and append to DB
    debug!("Recieved Shorten request: {}", shorten_req.url);
    let resp = ShortenedUrlData::new(&shorten_req.url);
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(resp)
}
