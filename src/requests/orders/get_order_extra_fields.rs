use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OrderExtraField {
    pub extra_field_id: i64,
    pub name: String,
    pub editor_type: String,
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrderExtraFieldsResponse {
    pub extra_fields: Vec<OrderExtraField>,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrderExtraFields {}

impl RequestTrait<GetOrderExtraFieldsResponse> for GetOrderExtraFields {
    const METHOD: &'static str = "getOrderExtraFields";
}
