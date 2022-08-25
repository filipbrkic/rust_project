use actix_web::error;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

use super::schema::*;

#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct VehicleBrands {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = vehicle_brands)]
pub struct NewBrand {
    pub name: String,
    pub description: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = vehicle_brands)]
pub struct NewVehicleBrand<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct Owners {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = owners)]
pub struct NewOwner {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = owners)]
pub struct NewOwners<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}

#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct VehicleModels {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = vehicle_models)]
pub struct NewModel {
    pub name: String,
    pub description: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = vehicle_models)]
pub struct NewVehicleModels<'a> {
    pub name: &'a str,
    pub description: &'a str,
}


#[derive(Debug, Display, Error)]
#[display(fmt = "Error: {}", name)]
pub struct MyError {
    pub name: &'static str,   
}

impl error::ResponseError for MyError {}