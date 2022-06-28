use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddInventoryWarehouseResponse {
    pub warehouse_id: i64,
}

/// The method allows you to add a new warehouse available in BaseLinker catalogues.
///
/// Adding a warehouse with the same identifier again will cause updates of the previously saved warehouse.
///
/// The method does not allow editing warehouses created automatically for the purpose of keeping external stocks of shops, wholesalers etc.
///
/// Such warehouse may be later used in addInventory method.
#[derive(Serialize, Deserialize, Debug)]
pub struct AddInventoryWarehouse {
    pub warehouse_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub stock_edition: bool,
}

impl RequestTrait<AddInventoryWarehouseResponse> for AddInventoryWarehouse {
    const METHOD: &'static str = "addInventoryWarehouse";
}
