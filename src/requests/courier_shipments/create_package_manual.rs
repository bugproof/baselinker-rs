use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The method allows you to enter the shipping number and the name of the courier to the order (function used only to add shipments created outside BaseLinker)
#[derive(Serialize, Deserialize)]
pub struct CreatePackageManual {
    /// Order identifier
    pub order_id: i64,
    /// Courier code (courier code retrieved with getCourierList or custom courier name)
    pub courier_code: String,
    /// Shipping number (consignment number)
    pub package_number: String,
    /// Date of dispatch (unix time format)
    #[serde(with = "ts_seconds")]
    pub pickup_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePackageManualResponse {
    /// Shipment ID
    pub package_id: i64,
    /// Shipping number (consignment number)
    pub package_number: String,
}

impl RequestTrait<CreatePackageManualResponse> for CreatePackageManual {
    const METHOD: &'static str = "createPackageManual";
}
