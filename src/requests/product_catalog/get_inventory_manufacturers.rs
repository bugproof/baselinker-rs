use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct InventoryManufacturer {
    pub manufacturer_id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryManufacturersResponse {
    pub manufacturers: Vec<InventoryManufacturer>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryManufacturers {}

impl RequestTrait<GetInventoryManufacturersResponse> for GetInventoryManufacturers { const METHOD: &'static str = "getInventoryManufacturers"; }