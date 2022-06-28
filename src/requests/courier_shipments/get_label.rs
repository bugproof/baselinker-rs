use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLabelResponse {
    /// Label file extension (pdf, html, gif, png, epl, zpl, dpl).
    pub extension: String,
    /// Label encoded with base64 algorithm.
    pub label: String,
}

/// The method allows you to download a shipping label (consignment) for a selected shipment.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLabel {
    /// Courier code
    pub courier_code: String,
    /// Shipment ID, optional if package_number was provided
    pub package_id: Option<i64>,
    /// Shipping number (consignment number), optional if package_id was provided
    pub package_number: Option<i64>,
}

impl RequestTrait<GetLabelResponse> for GetLabel {
    const METHOD: &'static str = "getLabel";
}
