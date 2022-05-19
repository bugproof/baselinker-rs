use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to remove manufacturer from BaseLinker catalog
#[derive(Serialize, Deserialize)]
pub struct DeleteInventoryManufacturer {
    /// The ID of the manufacturer removed from BaseLinker warehouse.
    pub manufacturer_id: i64,
}

impl RequestTrait<IgnoredAny> for DeleteInventoryManufacturer {
    const METHOD: &'static str = "deleteInventoryManufacturer";
}
