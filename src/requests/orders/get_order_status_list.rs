use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    /// status identifier
    pub id: i64,
    /// status name (basic)
    pub name: String,
    /// long status name (displayed to the customer on the order page)
    pub name_for_customer: String,
    /// status color in hex
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderStatusListResponse {
    pub statuses: Vec<OrderStatus>,
}

/// The method allows you to download order statuses created by the customer in the BaseLinker order manager.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderStatusList {}

impl RequestTrait<GetOrderStatusListResponse> for GetOrderStatusList {
    const METHOD: &'static str = "getOrderStatusList";
}
