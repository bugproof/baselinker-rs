use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to remove the warehouse available in BaseLinker catalogues.
///
/// The method does not allow to remove warehouses created automatically for the purpose of keeping external stocks of shops, wholesalers etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteInventoryWarehouse {
    /// ID of the warehouse
    pub warehouse_id: i64,
}

impl RequestTrait<IgnoredAny> for DeleteInventoryWarehouse {
    const METHOD: &'static str = "deleteInventoryWarehouse";
}
