use actix_web::{App, HttpServer};
mod routes;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::load();  // Load env settings
    HttpServer::new(|| {
        App::new()
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

