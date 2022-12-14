use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenv::dotenv;
use handlers::*;
use std::env;
#[macro_use]
extern crate diesel;
extern crate dotenv;
mod handlers;
pub mod models;
pub mod schema;
mod tests;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("1");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    println!("2");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    println!("Server started");
    
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
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
