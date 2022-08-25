use crate::diesel::RunQueryDsl;
use crate::models::{MyError, NewBrand, NewVehicleBrand, VehicleBrands};
use crate::{schema, DbPool};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};

#[get("/brands/{id}")]
pub async fn get_vehicle_brands_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<u32>,
) -> Result<impl Responder, MyError> {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::diesel::QueryDsl;
    use crate::schema::vehicle_brands::dsl::vehicle_brands;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let results = vehicle_brands
        .find(id)
        .load::<VehicleBrands>(&mut connection)
        .expect("Error getting vehicle brand by ID");

    if results.len() == 0 {
        Err(MyError {
            name: "Error getting vehicle brand by ID",
        })
    } else {
        Ok(HttpResponse::Ok().json(results))
    }
}

#[get("/brands")]
pub async fn get_vehicle_brands(pool: web::Data<DbPool>) -> Result<impl Responder, MyError> {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::vehicle_brands::dsl::vehicle_brands;

    let results = vehicle_brands
        .load::<VehicleBrands>(&mut connection)
        .expect("Error loading vehicle brands!");

    if results.len() == 0 {
        Err(MyError {
            name: "Error loading vehicle brands!",
        })
    } else {
        Ok(HttpResponse::Ok().json(results))
    }
}

#[post("/brands")]
pub async fn post_vehicle_brand(
    pool: web::Data<DbPool>,
    info: web::Json<NewBrand>,
) -> Result<impl Responder, Error> {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    let new_vehicle = NewVehicleBrand {
        name: &info.name,
        description: &info.description,
    };
    use schema::vehicle_brands;

    let result = diesel::insert_into(vehicle_brands::table)
        .values(&new_vehicle)
        .get_result::<VehicleBrands>(&mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[put("/brands/{id}")]
pub async fn update_vehicle_brands(
    path: web::Path<u32>,
    info: web::Json<NewBrand>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, MyError> {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::diesel::QueryDsl;
    use crate::schema::vehicle_brands::dsl::vehicle_brands;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let new_vehicle = NewVehicleBrand {
        name: &info.name,
        description: &info.description,
    };

    let result = diesel::update(vehicle_brands.find(id))
        .set(&new_vehicle)
        .get_result::<VehicleBrands>(&mut connection)
        .expect(&format!("Unable to find vehicle brand with id: {}", id));

    if info.name.len() == 0 || info.description.len() == 0 {
        Err(MyError {
            name: "Error updating vehicle brand",
        })
    } else {
        Ok(HttpResponse::Ok().json(result))
    }
}

#[delete("/brands/{id}")]
pub async fn delete_vehicle_brand(
    path: web::Path<u32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, MyError> {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    use crate::diesel::QueryDsl;
    use crate::schema::vehicle_brands::dsl::vehicle_brands;

    let result = diesel::delete(vehicle_brands.find(id))
        .execute(&mut connection)
        .expect("Error deleting vehicle brand!");

    if result == 1 {
        Ok(HttpResponse::Ok())
    } else {
        Err(MyError {
            name: "Error deleting vehicle brand",
        })
    }
}
