use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to mark orders with a receipt already issued.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetOrderReceipt {
    pub receipt_id: i64,
    pub receipt_nr: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub printer_error: bool,
}

impl RequestTrait<IgnoredAny> for SetOrderReceipt {
    const METHOD: &'static str = "setOrderReceipt";
}
