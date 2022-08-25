use crate::diesel::RunQueryDsl;
use crate::models::{NewOwner, NewOwners, Owners};
use crate::{schema, DbPool};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/owners/{id}")]
pub async fn get_owners_by_id(path: web::Path<u32>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::diesel::QueryDsl;
    use crate::schema::owners::dsl::owners;

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let results = owners
        .find(id)
        .load::<Owners>(&mut connection)
        .expect("Error getting owners by ID");

    if results.len() == 0 {
        HttpResponse::NotFound().json(results)
    } else {
        HttpResponse::Ok().json(results)
    }
}

#[get("/owners")]
pub async fn get_owners(pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::owners::dsl::owners;

    let results = owners
        .load::<Owners>(&mut connection)
        .expect("Error loading owners");

    if results.len() == 0 {
        HttpResponse::NotFound().json(results)
    } else {
        HttpResponse::Ok().json(results)
    }
}

#[post("/owners")]
pub async fn post_owners(info: web::Json<NewOwner>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    let new_owners = NewOwners {
        first_name: &info.first_name,
        last_name: &info.last_name,
    };

    use schema::owners;

    let result = diesel::insert_into(owners::table)
        .values(&new_owners)
        .get_result::<Owners>(&mut connection)
        .expect("Error saving new owners");

    HttpResponse::Ok().json(result)
}

#[put("/owners/{id}")]
pub async fn update_owners(path: web::Path<u32>, info: web::Json<NewOwner>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    let new_owners = NewOwners {
        first_name: &info.first_name,
        last_name: &info.last_name,
    };

    use crate::diesel::QueryDsl;
    use crate::schema::owners::dsl::owners;

    let result = diesel::update(owners.find(id))
        .set(&new_owners)
        .get_result::<Owners>(&mut connection)
        .expect(&format!("Unable to find brand with id: {}", id));

    HttpResponse::Ok().json(result)
}

#[delete("/owners/{id}")]
pub async fn delete_owners(path: web::Path<u32>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Couldn't get db connection from pool");

    let path_to_string = &path.into_inner().to_string();
    let id = &path_to_string.parse::<i32>().unwrap();

    use crate::diesel::QueryDsl;
    use crate::schema::owners::dsl::owners;
    let result = diesel::delete(owners.find(id))
        .execute(&mut connection)
        .expect("Error deleting owners!");

    if result == 1 {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}
