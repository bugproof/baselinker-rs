use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddInventoryManufacturerResponse {
    /// ID of a created or updated manufacturer
    pub manufacturer_id: i64,
}

/// The method allows you to add a manufacturer to the BaseLinker catalog.
///
/// Adding a manufacturer with the same identifier again, updates the previously saved manufacturer
#[derive(Serialize, Deserialize, Debug)]
pub struct AddInventoryManufacturer {
    /// Manufacturer ID provided in case of an update. Should be blank when creating a new manufacturer.
    pub manufacturer_id: Option<i64>,
    /// Manufacturer name
    pub name: String,
}

impl RequestTrait<AddInventoryManufacturerResponse> for AddInventoryManufacturer {
    const METHOD: &'static str = "addInventoryManufacturer";
}
