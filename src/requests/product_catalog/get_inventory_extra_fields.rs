use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct InventoryExtraField {
    pub extra_field_id: i64,
    pub name: String,
    pub kind: i64,
    pub editor_type: String,
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryExtraFieldsResponse {
    pub extra_fields: Vec<InventoryExtraField>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryExtraFields {}

impl RequestTrait<GetInventoryExtraFieldsResponse> for GetInventoryExtraFields { const METHOD: &'static str = "getInventoryExtraFields"; }