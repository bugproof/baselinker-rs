use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct InventoryIntegration {
    pub langs: Vec<String>,
    pub accounts: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryIntegrationsResponse {
    pub integrations: Vec<InventoryIntegration>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryIntegrations {
    pub inventory_id: i64,
}

impl RequestTrait<GetInventoryIntegrationsResponse> for GetInventoryIntegrations { const METHOD: &'static str = "getInventoryIntegrations"; }