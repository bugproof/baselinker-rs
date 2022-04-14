use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

/// The method allows you to add a payment to the order.
#[derive(Serialize, Deserialize)]
pub struct SetOrderPayment {
    pub order_id: i64,
    /// The amount of the payment.
    ///
    /// The value changes the current payment in the order (not added to the previous value).
    ///
    /// If the amount matches the order value, the order will be marked as paid.
    pub payment_done: f64,
    #[serde(with = "ts_seconds")]
    pub payment_date: DateTime<Utc>,
    pub payment_comment: String,
    pub external_payment_id: Option<String>,
}

impl RequestTrait<IgnoredAny> for SetOrderPayment { const METHOD: &'static str = "setOrderPayment"; }