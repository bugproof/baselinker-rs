use serde::{Serialize, Deserialize};
use serde::de::IgnoredAny;
use crate::common::RequestTrait;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

/// The method allows you to mark orders with a receipt already issued.
#[derive(Serialize, Deserialize)]
pub struct SetOrderReceipt {
    pub receipt_id: i64,
    pub receipt_nr: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub printer_error: bool,
}

impl RequestTrait<IgnoredAny> for SetOrderReceipt { const METHOD: &'static str = "setOrderReceipt"; }