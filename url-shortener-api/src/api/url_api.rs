use crate::{models::shorten_request::ShortenRequest, repository::mongodb_repo::MongoRepo};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};
use log::debug;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

#[post("/url/shorten")]
pub async fn shorten(db: Data<MongoRepo>, shorten_req: Json<ShortenRequest>) -> HttpResponse {
    // TODO Check and append to DB
    debug!("Recieved Shorten request: {}", shorten_req.url);
    let url_detail = db.create_shortened_url(shorten_req.url.to_owned()).await;
    match url_detail {
        Ok(url) => HttpResponse::Created().json(url),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
