use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderSourcesResponse {
    pub sources: HashMap<String, HashMap<i64, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderSources {}
impl RequestTrait<GetOrderSourcesResponse> for GetOrderSources {
    const METHOD: &'static str = "getOrderSources";
}
