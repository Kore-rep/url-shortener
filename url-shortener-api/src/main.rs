mod api;
mod constants;
mod models;
mod repository;

use actix_web::{middleware, web::Data, App, HttpServer};
use api::url_api::{get_url, hello, shorten};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //create_url_key_index(&client).await;
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    pretty_env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db_data.clone())
            .service(hello)
            .service(shorten)
            .service(get_url)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
