use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod db;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection().await;
    println!("Database connection established");
    
    println!("Starting server on PORT 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}