use crate::diesel::RunQueryDsl;
use crate::models::{NewModel, NewVehicleModels, VehicleModels};
use crate::{schema, DbPool};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/models/{id}")]
pub async fn get_vehicle_models_by_id(
    path: web::Path<u32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::diesel::QueryDsl;
    use crate::schema::vehicle_models::dsl::vehicle_models;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let results = vehicle_models
        .find(id)
        .load::<VehicleModels>(&mut connection)
        .expect("Error getting vehicle model by ID");

    if results.len() == 0 {
        HttpResponse::NotFound().json(results)
    } else {
        HttpResponse::Ok().json(results)
    }
}

#[get("/models")]
pub async fn get_vehicle_models(pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::vehicle_models::dsl::vehicle_models;

    let results = vehicle_models
        .load::<VehicleModels>(&mut connection)
        .expect("Error loading vehicle models!");

    if results.len() == 0 {
        HttpResponse::NotFound().json(results)
    } else {
        HttpResponse::Ok().json(results)
    }
}

#[post("/models")]
pub async fn post_vehicle_model(
    info: web::Json<NewModel>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    let new_vehicle = NewVehicleModels {
        name: &info.name,
        description: &info.description,
    };

    use schema::vehicle_models;

    let result = diesel::insert_into(vehicle_models::table)
        .values(&new_vehicle)
        .get_result::<VehicleModels>(&mut connection)
        .expect("Error saving new vehicle");

    HttpResponse::Ok().json(result)
}

#[put("/models/{id}")]
pub async fn update_vehicle_models(
    path: web::Path<u32>,
    info: web::Json<NewModel>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::diesel::QueryDsl;
    use crate::schema::vehicle_models::dsl::vehicle_models;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let new_vehicle = NewVehicleModels {
        name: &info.name,
        description: &info.description,
    };

    let result = diesel::update(vehicle_models.find(id))
        .set(&new_vehicle)
        .get_result::<VehicleModels>(&mut connection)
        .expect(&format!("Unable to find vehicle model with id: {}", id));

    HttpResponse::Ok().json(result)
}

#[delete("/models/{id}")]
pub async fn delete_vehicle_model(path: web::Path<u32>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    use crate::diesel::QueryDsl;
    use crate::schema::vehicle_models::dsl::vehicle_models;
    let result = diesel::delete(vehicle_models.find(id))
        .execute(&mut connection)
        .expect("Error deleting vehicle model!");

    if result == 1 {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}
