use crate::common::RequestTrait;
use crate::serialization::inconsistent_bool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    /// The field ID
    pub id: String,
    /// The field name
    pub name: String,
    /// Field type (available select, checkbox, text, date)
    #[serde(rename = "type")]
    pub field_type: String,
    /// Additional field description
    pub desc: Option<String>,
    /// List of available options (appears for select, checkbox).
    ///
    /// The key to each element is the option id (varchar)
    ///
    /// The value is the option name (varchar)
    pub options: Option<HashMap<String, String>>,
    /// List of additional fields that are available for the selected option.
    ///
    /// The key for each element is (varchar) - id of the option for which additional fields (varchar) are to be available
    ///
    /// The value is the list of fields that are available for this option (array)
    pub show_field: Option<HashMap<String, String>>,
    /// Default value for a field
    pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageField {
    /// The field ID
    pub id: String,
    /// The field name
    pub name: String,
    /// Field type (available select, checkbox, text)
    #[serde(rename = "type")]
    pub package_field_type: String,
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "default")]
    pub package_field_default: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCourierFieldsResponse {
    /// Does the courier support multiple shipments.
    #[serde(deserialize_with = "inconsistent_bool")]
    pub multi_packages: bool,
    /// An array with a list of fields to create a shipment
    pub fields: Vec<Field>,
    /// An array with a list of fields to create packages
    pub package_fields: Vec<PackageField>,
}

/// The method allows you to retrieve the form fields for creating shipments for the selected courier.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCourierFields {
    /// Courier code
    pub courier_code: String,
}

impl RequestTrait<GetCourierFieldsResponse> for GetCourierFields {
    const METHOD: &'static str = "getCourierFields";
}
