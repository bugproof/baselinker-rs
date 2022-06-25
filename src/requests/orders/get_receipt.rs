use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReceiptItem {
    pub name: String,
    pub price_brutto: Decimal,
    pub tax_rate: Decimal,
    pub quantity: i64,
    pub sku: String,
    pub ean: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetReceiptResponse {
    pub receipt_id: i64,
    pub series_id: i64,
    pub receipt_full_nr: String,
    pub year: i64,
    pub month: i64,
    pub sub_id: i64,
    pub order_id: i64,
    #[serde(with = "ts_seconds")]
    pub date_add: DateTime<Utc>,
    pub payment_method: String,
    pub nip: String,
    pub currency: String,
    pub total_price_brutto: Decimal,
    pub external_receipt_number: String,
    pub exchange_currency: String,
    pub exchange_rate: Decimal,
    pub exchange_date: String,
    pub exchange_info: String,
    pub items: Vec<ReceiptItem>,
}

/// The method allows you to retrieve a single receipt from the BaseLinker order manager.
///
/// To retrieve a list of new receipts (when integrating a fiscal printer), use the getNewReceipts method.
#[derive(Serialize, Deserialize)]
pub struct GetReceipt {
    // TODO: union/enum?
    /// Receipt ID. Not required if order_id is provided.
    pub receipt_id: Option<i64>,
    /// Order ID. Not required if receipt_id is provided.
    pub order_id: Option<i64>,
}

impl RequestTrait<GetReceiptResponse> for GetReceipt {
    const METHOD: &'static str = "getReceipt";
}
