use std::env;

use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};

use crate::{
    constants::{COLL_NAME, DB_NAME},
    models::shortened_url_model::ShortenedUrlData,
};

pub struct MongoRepo {
    col: Collection<ShortenedUrlData>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let db_uri = env::var("MONGO_URI").expect("MONGO_URI is empty");
        let client = Client::with_uri_str(db_uri)
            .await
            .expect("failed to connect");
        let db = client.database(DB_NAME);
        let col: Collection<ShortenedUrlData> = db.collection(COLL_NAME);
        MongoRepo { col }
    }

    pub async fn create_shortened_url(&self, new_url: String) -> Result<InsertOneResult, Error> {
        let new_doc = ShortenedUrlData::new(&new_url);
        let url = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating shortened URL");
        Ok(url)
    }
}
