use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryIntegration {
    pub langs: Vec<String>,
    pub accounts: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryIntegrationsResponse {
    pub integrations: Vec<InventoryIntegration>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryIntegrations {
    pub inventory_id: i64,
}

impl RequestTrait<GetInventoryIntegrationsResponse> for GetInventoryIntegrations {
    const METHOD: &'static str = "getInventoryIntegrations";
}
