use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to remove the product from BaseLinker catalog.
#[derive(Serialize, Deserialize)]
pub struct DeleteInventoryProduct {
    /// BaseLinker catalogue product identifier
    pub product_id: i64,
}

impl RequestTrait<IgnoredAny> for DeleteInventoryProduct {
    const METHOD: &'static str = "deleteInventoryProduct";
}
