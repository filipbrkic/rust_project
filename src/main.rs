use actix_web::{App, HttpServer};
use diesel::{PgConnection, Connection};
use dotenv::dotenv;
use std::env::{self};

#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod models;
pub mod schema;
pub mod brand_handlers;
use crate::brand_handlers::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(get_vehicle_brands_by_id)
            .service(get_vehicle_brands)
            .service(post_vehicle_brand)
            .service(delete_vehicle_brand)
            .service(update_vehicle_brands)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}