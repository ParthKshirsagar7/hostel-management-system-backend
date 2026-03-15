use actix_web::{web, App, HttpServer};

mod db;
mod models;
mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection().await;

    println!("Database connection established");
    println!("Starting server on PORT 8080");

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .error_handler(utils::json_error_handler);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(json_config)
            .service(handlers::health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}