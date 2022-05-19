use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to remove the price group from BaseLinker storage.
#[derive(Serialize, Deserialize)]
pub struct DeleteInventoryPriceGroup {
    /// Price group identifier
    pub price_group_id: i64,
}

impl RequestTrait<IgnoredAny> for DeleteInventoryPriceGroup {
    const METHOD: &'static str = "deleteInventoryPriceGroup";
}
