use crate::diesel::RunQueryDsl;
use crate::establish_connection;
use crate::models::{NewVehicleBrand, VehicleBrands};
use actix_web::{get, post, HttpResponse, Responder, delete, put, web};

#[get("/brands/{id}")]
pub async fn get_vehicle_brands_by_id(path: web::Path<u32>) -> impl Responder {
    let connection = establish_connection();

    use crate::schema::vehicle_brands::dsl::vehicle_brands;
    use crate::diesel::QueryDsl;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let results = vehicle_brands.find(id)
        .load::<VehicleBrands>(&connection)
        .expect("Error getting vehicle brand by ID");

    let serialized = serde_json::to_string(&results).unwrap();

    if serialized == "[]" {
        HttpResponse::NotFound().body(serialized)
    } else {
        HttpResponse::Ok().body(serialized)
    }
}

#[get("/brands")]
pub async fn get_vehicle_brands() -> impl Responder {
    let connection = establish_connection();

    use crate::schema::vehicle_brands::dsl::vehicle_brands;

    let results = vehicle_brands
        .load::<VehicleBrands>(&connection)
        .expect("Error loading vehicle brands!");
    let serialized = serde_json::to_string(&results).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[post("/brands")]
pub async fn post_vehicle_brand(req_body: String) -> impl Responder {
    let connection = establish_connection();

    let id_part: Vec<&str> = req_body.split("\"id\"").collect();
    let id_part = id_part[1].trim_start();
    let id_part: Vec<&str> = id_part.split("\n--------").collect();
    let id_part = id_part[0].trim_end();
    
    let name_part: Vec<&str> = req_body.split("\"name\"").collect();
    let name_part = name_part[1].trim_start();
    let name_part: Vec<&str> = name_part.split("\n--------").collect();
    let name_part = name_part[0].trim_end();

    let description_part: Vec<&str> = req_body.split("\"description\"").collect();
    let description_part = description_part[1].trim_start();
    let description_part: Vec<&str> = description_part.split("\n--------").collect();
    let description_part = description_part[0].trim_end();

    let new_vehicle = NewVehicleBrand {
        id: &id_part.parse::<i32>().unwrap(),
        name: &name_part.to_string(),
        description: &description_part.to_string()
    };
    use crate::schema::vehicle_brands::dsl::vehicle_brands;

    let create_vehicle_brand = diesel::insert_into(vehicle_brands)
        .values(&new_vehicle)
        .get_result::<VehicleBrands>(&connection)
        .expect("Error saving new vehicle");

    let serialized = serde_json::to_string(&create_vehicle_brand).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[put("/brands")]
pub async fn update_vehicle_brands(req_body: String) -> impl Responder {
    let connection = establish_connection();

    let id_part: Vec<&str> = req_body.split("\"id\"").collect();
    let id_part = id_part[1].trim_start();
    let id_part: Vec<&str> = id_part.split("\n--------").collect();
    let id_part = id_part[0].trim_end();
    
    let name_part: Vec<&str> = req_body.split("\"name\"").collect();
    let name_part = name_part[1].trim_start();
    let name_part: Vec<&str> = name_part.split("\n--------").collect();
    let name_part = name_part[0].trim_end();

    let description_part: Vec<&str> = req_body.split("\"description\"").collect();
    let description_part = description_part[1].trim_start();
    let description_part: Vec<&str> = description_part.split("\n--------").collect();
    let description_part = description_part[0].trim_end();
    
    use crate::schema::vehicle_brands::dsl::vehicle_brands;
    use crate::diesel::QueryDsl;

    let new_vehicle = NewVehicleBrand {
        id: &id_part.parse::<i32>().unwrap(),
        name: &name_part.to_string(),
        description: &description_part.to_string()
    };

    let id = &id_part.parse::<i32>().unwrap();

    let update_vehicle_brands = diesel::update(vehicle_brands.find(id))
        .set(&new_vehicle)
        .get_result::<VehicleBrands>(&connection)
        .expect(&format!("Unable to find vehicle brand with id: {}", id));

    let serialized = serde_json::to_string(&update_vehicle_brands).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[delete("/brands/{id}")]
pub async fn delete_vehicle_brand(path: web::Path<u32>) -> impl Responder {

    let connection = establish_connection();

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();
    
    use crate::schema::vehicle_brands::dsl::vehicle_brands;
    use crate::diesel::QueryDsl;
    let brand_deleted = diesel::delete(vehicle_brands.find(id))
        .execute(&connection)
        .expect("Error deleting vehicle brand!");

    let serialized = serde_json::to_string(&brand_deleted).unwrap();
    let result = serialized.parse::<i32>().unwrap();
    if result == 1{
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}