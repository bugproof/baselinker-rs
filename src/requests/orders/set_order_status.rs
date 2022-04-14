use serde::{Serialize, Deserialize};
use serde::de::IgnoredAny;
use crate::common::RequestTrait;

/// The method allows you to change order status.
#[derive(Serialize, Deserialize)]
pub struct SetOrderStatus {
    pub order_id: i64,
    /// Status ID number. The status list can be retrieved using getOrderStatusList.
    pub status_id: i64,
}

impl RequestTrait<IgnoredAny> for SetOrderStatus { const METHOD: &'static str = "setOrderStatus"; }