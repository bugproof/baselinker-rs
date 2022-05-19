use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to delete a catalog from BaseLinker storage.
#[derive(Serialize, Deserialize)]
pub struct DeleteInventory {
    /// Catalog ID. The list of identifiers can be retrieved using the method getInventories.
    pub inventory_id: i64,
}

impl RequestTrait<IgnoredAny> for DeleteInventory {
    const METHOD: &'static str = "deleteInventory";
}
