use mongodb::bson::oid::ObjectId;
use serde::Serialize;

use crate::constants::REDIRECT_SERVER_URL;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
#[derive(Serialize)]
pub struct ShortenedUrlData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub key: String,
    pub short_url: String,
    pub long_url: String,
}

impl ShortenedUrlData {
    pub fn new(url: &String) -> Self {
        let url_hash = hash_url::<String>(url);
        Self {
            id: None,
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
