mod api;
mod constants;
mod models;
mod repository;

use actix_web::{middleware, web::Data, App, HttpServer};
use api::url_api::{hello, shorten};
use repository::mongodb_repo::MongoRepo;

// async fn create_url_key_index(client: &Client) {
//     let options = IndexOptions::builder().unique(true).build();
//     let model = IndexModel::builder()
//         .keys(doc! {"key": 1})
//         .options(options)
//         .build();
//     client
//         .database(DB_NAME)
//         .collection::<ShortenedUrlData>(COL_NAME)
//         .create_index(model, None)
//         .await
//         .expect("Creating index should succeed");
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //create_url_key_index(&client).await;
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db_data.clone())
            .service(hello)
            .service(shorten)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
