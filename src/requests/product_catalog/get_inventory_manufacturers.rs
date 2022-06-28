use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryManufacturer {
    pub manufacturer_id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryManufacturersResponse {
    pub manufacturers: Vec<InventoryManufacturer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryManufacturers {}

impl RequestTrait<GetInventoryManufacturersResponse> for GetInventoryManufacturers {
    const METHOD: &'static str = "getInventoryManufacturers";
}
