use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to remove a specific product from the order.
#[derive(Serialize, Deserialize)]
pub struct DeleteOrderProduct {
    /// Order ID from BaseLinker order manager.
    pub order_id: i64,
    /// Order item ID from BaseLinker order manager.
    pub order_product_id: i64,
}

impl RequestTrait<IgnoredAny> for DeleteOrderProduct {
    const METHOD: &'static str = "deleteOrderProduct";
}
