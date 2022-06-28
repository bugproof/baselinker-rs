use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to change order status.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetOrderStatus {
    pub order_id: i64,
    /// Status ID number. The status list can be retrieved using getOrderStatusList.
    pub status_id: i64,
}

impl RequestTrait<IgnoredAny> for SetOrderStatus {
    const METHOD: &'static str = "setOrderStatus";
}
