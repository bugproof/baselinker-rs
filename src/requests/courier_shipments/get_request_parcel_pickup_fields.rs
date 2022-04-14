use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct RequestParcelPickupField {
    /// The field ID
    pub id: String,
    /// The field name
    pub name: String,
    /// Field type (available select, checkbox, text, date)
    pub r#type: String,
    /// Additional field description
    pub desc: Option<String>,
    /// List of available options (appears for select, checkbox).
    ///
    /// The key to each element is the option id (varchar)
    ///
    /// The value is the option name (varchar)
    pub options: Option<HashMap<String, String>>,
    /// Default value for a field
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetRequestParcelPickupFieldsResponse {
    /// An array with a list of additional fields to request parcel pickup
    pub fields: Vec<RequestParcelPickupField>,
}

/// The method allows you to retrieve additional fields for a parcel pickup request.
#[derive(Serialize, Deserialize)]
pub struct GetRequestParcelPickupFields {
    /// Courier code
    pub courier_code: String,
}

impl RequestTrait<GetRequestParcelPickupFieldsResponse> for GetRequestParcelPickupFields { const METHOD: &'static str = "getRequestParcelPickupFields"; }