use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Serialize)]
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

#[derive(Deserialize)]
pub struct CreatePackageManualResponse {
    pub status: String,
    pub package_id: i64,
    pub package_number: String,
}

impl RequestTrait<CreatePackageManualResponse> for CreatePackageManual { const METHOD: &'static str = "createPackageManual"; }