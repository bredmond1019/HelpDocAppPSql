use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::{error, info};
use log4rs;
use std::env;
use std::sync::Arc;
// use std::process::Command;

use db::DbPool;

pub mod db;
pub mod routes;
pub mod schema;
pub mod scripts;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");

    // Initialize logger
    match log4rs::init_file("log4rs.yaml", Default::default()) {
        Ok(_) => info!("Logger initialized successfully"),
        Err(e) => error!("Failed to initialize logger: {}", e),
    }

    info!("Starting application");

    let pool: DbPool = db::init_pool();
    let arc_pool = Arc::new(pool.clone());

    scripts::migrate::migrate_data()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Start the server
    info!("Server listening on 127.0.0.1:3000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(arc_pool.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
    .map_err(|e| {
        error!("Server error: {:?}", e);
        e
    })
}
