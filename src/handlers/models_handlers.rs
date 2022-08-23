use crate::diesel::RunQueryDsl;
use crate::{schema, establish_connection};
use crate::models::{NewVehicleModels, VehicleModels};
use actix_web::{get, post, HttpResponse, Responder, delete, put, web};
use rand::Rng;

#[get("/models/{id}")]
pub async fn get_vehicle_models_by_id(path: web::Path<u32>) -> impl Responder {
    let connection = establish_connection();

    use crate::schema::vehicle_models::dsl::vehicle_models;
    use crate::diesel::QueryDsl;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let results = vehicle_models.find(id)
        .load::<VehicleModels>(&connection)
        .expect("Error getting vehicle model by ID");

    let serialized = serde_json::to_string(&results).unwrap();

    if serialized == "[]" {
        HttpResponse::NotFound().body(serialized)
    } else {
        HttpResponse::Ok().body(serialized)
    }
}

#[get("/models")]
pub async fn get_vehicle_models() -> impl Responder {
    let connection = establish_connection();

    use crate::schema::vehicle_models::dsl::vehicle_models;

    let results = vehicle_models
        .load::<VehicleModels>(&connection)
        .expect("Error loading vehicle models!");
    let serialized = serde_json::to_string(&results).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[post("/models")]
pub async fn post_vehicle_model(req_body: String) -> impl Responder {
    let connection = establish_connection();

    let name_part: Vec<&str> = req_body.split("\"name\"").collect();
    let name_part = name_part[1].trim_start();
    let name_part: Vec<&str> = name_part.split("\n----").collect();
    let name_part = name_part[0].trim_end();

    let description_part: Vec<&str> = req_body.split("\"description\"").collect();
    let description_part = description_part[1].trim_start();
    let description_part: Vec<&str> = description_part.split("\n----").collect();
    let description_part = description_part[0].trim_end();

    let id = rand::thread_rng().gen_range(0..1000000);

    let new_vehicle = NewVehicleModels {
        id: &id,
        name: &name_part.to_string(),
        description: &description_part.to_string()
    };

    use schema::vehicle_models;

    let create_vehicle_model = diesel::insert_into(vehicle_models::table)
        .values(&new_vehicle)
        .get_result::<VehicleModels>(&connection)
        .expect("Error saving new vehicle");

    let serialized = serde_json::to_string(&create_vehicle_model).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[put("/models")]
pub async fn update_vehicle_models(req_body: String) -> impl Responder {
    let connection = establish_connection();

    let id_part: Vec<&str> = req_body.split("\"id\"").collect();
    let id_part = id_part[1].trim_start();
    let id_part: Vec<&str> = id_part.split("\n----").collect();
    let id_part = id_part[0].trim_end();
    
    let name_part: Vec<&str> = req_body.split("\"name\"").collect();
    let name_part = name_part[1].trim_start();
    let name_part: Vec<&str> = name_part.split("\n----").collect();
    let name_part = name_part[0].trim_end();

    let description_part: Vec<&str> = req_body.split("\"description\"").collect();
    let description_part = description_part[1].trim_start();
    let description_part: Vec<&str> = description_part.split("\n----").collect();
    let description_part = description_part[0].trim_end();
    
    use crate::schema::vehicle_models::dsl::vehicle_models;
    use crate::diesel::QueryDsl;

    let new_vehicle = NewVehicleModels {
        id: &id_part.parse::<i32>().unwrap(),
        name: &name_part.to_string(),
        description: &description_part.to_string()
    };

    let id = &id_part.parse::<i32>().unwrap();

    let update_vehicle_models = diesel::update(vehicle_models.find(id))
        .set(&new_vehicle)
        .get_result::<VehicleModels>(&connection)
        .expect(&format!("Unable to find vehicle model with id: {}", id));

    let serialized = serde_json::to_string(&update_vehicle_models).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[delete("/models/{id}")]
pub async fn delete_vehicle_model(path: web::Path<u32>) -> impl Responder {

    let connection = establish_connection();

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();
    
    use crate::schema::vehicle_models::dsl::vehicle_models;
    use crate::diesel::QueryDsl;
    let model_deleted = diesel::delete(vehicle_models.find(id))
        .execute(&connection)
        .expect("Error deleting vehicle model!");

    let serialized = serde_json::to_string(&model_deleted).unwrap();
    let result = serialized.parse::<i32>().unwrap();
    if result == 1{
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}