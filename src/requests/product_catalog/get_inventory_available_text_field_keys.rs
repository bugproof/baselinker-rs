use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct GetInventoryAvailableTextFieldKeysResponse {
    pub text_field_keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryAvailableTextFieldKeys {
    pub inventory_id: i64,
}

impl RequestTrait<GetInventoryAvailableTextFieldKeysResponse> for GetInventoryAvailableTextFieldKeys { const METHOD: &'static str = "getInventoryAvailableTextFieldKeys"; }