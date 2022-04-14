use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct RequestParcelPickupResponse {
    /// The parcel pickup number provided by the courier API
    pub pickup_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestParcelPickupField {
    /// The field ID
    pub id: String,
    /// The field value
    pub value: String,
}

/// The method allows you to retrieve additional fields for a parcel pickup request.
#[derive(Serialize, Deserialize)]
pub struct RequestParcelPickup {
    /// Courier code
    pub courier_code: String,
    /// Array of shipments ID, optional if package_numbers was provided
    pub package_ids: Option<Vec<String>>,
    /// Array of shipments number (consignment number), optional if package_ids was provided
    pub package_numbers: Option<Vec<String>>,
    /// Courier API account id for the courier accounts retrieved from the request getCourierAccounts
    pub account_id: i64,
    /// List of form fields retrieved from the request getRequestParcelPickupFields
    pub fields: Vec<RequestParcelPickupField>,
}

impl RequestTrait<RequestParcelPickupResponse> for RequestParcelPickup { const METHOD: &'static str = "requestParcelPickup"; }