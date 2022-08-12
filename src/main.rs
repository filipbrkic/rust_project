use actix_web::{App, HttpServer};
use diesel::{PgConnection, Connection};
use dotenv::dotenv;
use handlers::*;
use std::env;

#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod models;
pub mod schema;
mod handlers;
mod vehicle_tests;

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

            .service(get_owners_by_id)
            .service(get_owners)
            .service(post_owners)
            .service(update_owners)
            .service(delete_owners)

            .service(get_vehicle_models_by_id)
            .service(get_vehicle_models)
            .service(post_vehicle_model)
            .service(update_vehicle_models)
            .service(delete_vehicle_model)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}