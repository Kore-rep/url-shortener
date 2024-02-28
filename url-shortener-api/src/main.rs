mod constants;
mod url;

use actix_web::{middleware, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(url::shorten)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
