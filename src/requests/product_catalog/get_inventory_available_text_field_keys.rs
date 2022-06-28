use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryAvailableTextFieldKeysResponse {
    pub text_field_keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryAvailableTextFieldKeys {
    pub inventory_id: i64,
}

impl RequestTrait<GetInventoryAvailableTextFieldKeysResponse>
    for GetInventoryAvailableTextFieldKeys
{
    const METHOD: &'static str = "getInventoryAvailableTextFieldKeys";
}
