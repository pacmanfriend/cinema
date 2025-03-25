mod cinemas;
mod films;
mod sessions;
mod tickets;
mod bookings;
mod models;
mod handlers;
mod db;
mod errors;
mod routes;

use actix_web::{App, HttpServer};
use db::init_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = init_db_pool().await
        .expect("Failed to create database connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(routes::config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}