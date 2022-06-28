use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProtocolResponse {
    /// Protocol file extension (pdf, html).
    pub extension: String,
    /// Protocol encoded with base64 algorithm.
    pub protocol: String,
}

/// The method allows you to download a parcel protocol for selected shipments if the protocol is available for chosen courier
#[derive(Serialize, Deserialize, Debug)]
pub struct GetProtocol {
    /// Courier code
    pub courier_code: String,
    /// Array of shipments ID, optional if package_numbers was provided
    pub package_ids: Option<Vec<i64>>,
    /// Array of shipments number (consignment number), optional if package_ids was provided
    pub package_numbers: Option<Vec<i64>>,
    /// Courier API account id for the courier accounts retrieved from the request getCourierAccounts
    pub account_id: i64,
}

impl RequestTrait<GetProtocolResponse> for GetProtocol {
    const METHOD: &'static str = "getProtocol";
}
