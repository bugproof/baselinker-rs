use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    /// total amount paid before the given change
    pub paid_before: Decimal,
    /// total amount paid after the change
    pub paid_after: Decimal,
    pub total_price: Decimal,
    pub currency: String,
    /// external payment identifier
    pub external_payment_id: String,
    /// date of change record (unix time format)
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderPaymentsHistoryResponse {
    pub payments: Vec<Payment>,
}

/// The method allows you to retrieve payment history for a selected order, including an external payment identifier from the payment gateway.
///
/// One order can have multiple payment history entries, caused by surcharges, order value changes, manual payment editing
#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderPaymentsHistory {
    pub order_id: i64,
    /// (false by default) Download full payment history, including order value change entries, manual order payment edits.
    ///
    /// False by default - only returns entries containing an external payment identifier (most commonly used)
    pub show_full_history: Option<bool>,
}

impl RequestTrait<GetOrderPaymentsHistoryResponse> for GetOrderPaymentsHistory {
    const METHOD: &'static str = "getOrderPaymentsHistory";
}
