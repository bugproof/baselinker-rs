use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryExtraField {
    pub extra_field_id: i64,
    pub name: String,
    pub kind: i64,
    pub editor_type: String,
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryExtraFieldsResponse {
    pub extra_fields: Vec<InventoryExtraField>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryExtraFields {}

impl RequestTrait<GetInventoryExtraFieldsResponse> for GetInventoryExtraFields {
    const METHOD: &'static str = "getInventoryExtraFields";
}
