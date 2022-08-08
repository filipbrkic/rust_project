#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct VehicleBrands {
    pub id: i32,
    pub name: String,
    pub description: String,
}

use serde::{Serialize, Deserialize};

use super::schema::vehicle_brands;

#[derive(Insertable, AsChangeset)]
#[table_name="vehicle_brands"]
pub struct NewVehicleBrand<'a> {
    pub id: &'a i32,
    pub name: &'a str,
    pub description: &'a str,
}