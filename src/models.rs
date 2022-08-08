use serde::{Serialize, Deserialize};

use super::schema::*;

#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct VehicleBrands {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name="vehicle_brands"]
pub struct NewVehicleBrand<'a> {
    pub id: &'a i32,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct Owners {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name="owners"]
pub struct NewOwners<'a> {
    pub id: &'a i32,
    pub first_name: &'a str,
    pub last_name: &'a str,
}

#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct VehicleModels {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name="vehicle_models"]
pub struct NewVehicleModels<'a> {
    pub id: &'a i32,
    pub name: &'a str,
    pub description: &'a str,
}