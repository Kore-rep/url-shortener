use crate::{models::shorten_request::ShortenRequest, repository::mongodb_repo::MongoRepo};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
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

#[get("/url/{key}")]
pub async fn get_url(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let key = path.into_inner();
    if key.is_empty() {
        return HttpResponse::BadRequest().body("Invalid key");
    }
    let shortened_url_detail = db.get_shortened_url(key).await;
    match shortened_url_detail {
        Ok(url) => HttpResponse::Ok().json(url),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
