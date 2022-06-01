use std::collections::HashMap;
use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to edit selected fields (e.g. address data, notes, etc.) of a specific order.
///
/// Only the fields that you want to edit should be given, other fields can be omitted in the request.
#[derive(Serialize, Deserialize)]
pub struct SetOrderFields {
    pub order_id: i64,
    pub admin_comments: Option<String>,
    pub user_comments: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_cod: Option<bool>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub user_login: Option<String>,
    pub delivery_method: Option<String>,
    pub delivery_price: Option<serde_json::Number>,
    pub delivery_fullname: Option<String>,
    pub delivery_company: Option<String>,
    pub delivery_address: Option<String>,
    pub delivery_postcode: Option<String>,
    pub delivery_city: Option<String>,
    pub delivery_country_code: Option<String>,
    pub delivery_point_id: Option<String>,
    pub delivery_point_name: Option<String>,
    pub delivery_point_address: Option<String>,
    pub delivery_point_postcode: Option<String>,
    pub delivery_point_city: Option<String>,
    pub invoice_fullname: Option<String>,
    pub invoice_nip: Option<String>,
    pub invoice_address: Option<String>,
    pub invoice_postcode: Option<String>,
    pub invoice_city: Option<String>,
    pub invoice_country_code: Option<String>,
    pub want_invoice: Option<bool>,
    pub custom_extra_fields: HashMap<String, serde_json::Value>,
    pub pick_state: Option<i64>,
    pub pack_state: Option<i64>,
}

impl RequestTrait<IgnoredAny> for SetOrderFields {
    const METHOD: &'static str = "setOrderFields";
}
