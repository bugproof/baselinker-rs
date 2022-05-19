use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddInventoryPriceGroupResponse {
    /// The ID number of added or updated price group.
    pub price_group_id: i64,
}

/// The method allows to create a price group in BaseLinker storage.
///
/// Providing a price group ID will update the existing price group.
///
/// Such price groups may be later assigned in addInventory method.
#[derive(Serialize, Deserialize)]
pub struct AddInventoryPriceGroup {
    /// Price group identifier
    pub price_group_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub currency: String,
}

impl RequestTrait<AddInventoryPriceGroupResponse> for AddInventoryPriceGroup {
    const METHOD: &'static str = "addInventoryPriceGroup";
}
