use crate::diesel::RunQueryDsl;
use crate::{schema, establish_connection};
use crate::models::{NewOwners, Owners};
use actix_web::{get, post, HttpResponse, Responder, delete, put, web};
use rand::Rng;

#[get("/owners/{id}")]
pub async fn get_owners_by_id(path: web::Path<u32>) -> impl Responder {
    let connection = establish_connection();

    use crate::schema::owners::dsl::owners;
    use crate::diesel::QueryDsl;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let results = owners.find(id)
        .load::<Owners>(&connection)
        .expect("Error getting owners by ID");

    let serialized = serde_json::to_string(&results).unwrap();

    if serialized == "[]" {
        HttpResponse::NotFound().body(serialized)
    } else {
        HttpResponse::Ok().body(serialized)
    }
}

#[get("/owners")]
pub async fn get_owners() -> impl Responder {
    let connection = establish_connection();

    use crate::schema::owners::dsl::owners;

    let results = owners
        .load::<Owners>(&connection)
        .expect("Error loading owners");

    let serialized = serde_json::to_string(&results).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[post("/owners")]
pub async fn post_owners(req_body: String) -> impl Responder {
    let connection = establish_connection();

    let first_name_part: Vec<&str> = req_body.split("\"first_name\"").collect();
    let first_name_part = first_name_part[1].trim_start();
    let first_name_part: Vec<&str> = first_name_part.split("\n----").collect();
    let first_name_part = first_name_part[0].trim_end();

    let last_name_part: Vec<&str> = req_body.split("\"last_name\"").collect();
    let last_name_part = last_name_part[1].trim_start();
    let last_name_part: Vec<&str> = last_name_part.split("\n----").collect();
    let last_name_part = last_name_part[0].trim_end();

    let id = rand::thread_rng().gen_range(0..1000000);

    let new_owners = NewOwners {
        id: &id,
        first_name: &first_name_part.to_string(),
        last_name: &last_name_part.to_string()
    };

    use schema::owners;

    let create_owners = diesel::insert_into(owners::table)
        .values(&new_owners)
        .get_result::<Owners>(&connection)
        .expect("Error saving new owners");

    let serialize = serde_json::to_string(&create_owners).unwrap();
    HttpResponse::Ok().body(serialize)
}

#[put("/owners")]
pub async fn update_owners(req_body: String) -> impl Responder {
    let connection = establish_connection();

    let id_part: Vec<&str> = req_body.split("\"id\"").collect();
    let id_part = id_part[1].trim_start();
    let id_part: Vec<&str> = id_part.split("\n--------").collect();
    let id_part = id_part[0].trim_end();

    let first_name_part: Vec<&str> = req_body.split("\"first_name\"").collect();
    let first_name_part = first_name_part[1].trim_start();
    let first_name_part: Vec<&str> = first_name_part.split("\n--------").collect();
    let first_name_part = first_name_part[0].trim_end();

    let last_name_part: Vec<&str> = req_body.split("\"last_name\"").collect();
    let last_name_part = last_name_part[1].trim_start();
    let last_name_part: Vec<&str> = last_name_part.split("\n--------").collect();
    let last_name_part = last_name_part[0].trim_end();

    let new_owners = NewOwners {
        id: &id_part.parse::<i32>().unwrap(),
        first_name: &first_name_part.to_string(),
        last_name: &last_name_part.to_string()
    };

    let id = &id_part.parse::<i32>().unwrap();

    use crate::schema::owners::dsl::owners;
    use crate::diesel::QueryDsl;

    let results = diesel::update(owners.find(id))
        .set(&new_owners)
        .get_result::<Owners>(&connection)
        .expect(&format!("Unable to find brand with id: {}", id));

    let serialized = serde_json::to_string(&results).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[delete("/owners/{id}")]
pub async fn delete_owners(path: web::Path<u32>) -> impl Responder {
    let connection = establish_connection();

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();
    
    use crate::schema::owners::dsl::owners;
    use crate::diesel::QueryDsl;
    let results = diesel::delete(owners.find(id))
        .execute(&connection)
        .expect("Error deleting owners!");

    let serialized = serde_json::to_string(&results).unwrap();
    let result = serialized.parse::<i32>().unwrap();
    if result == 1{
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}