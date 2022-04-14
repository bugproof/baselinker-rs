use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use crate::serialization::inconsistent_bool;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::de::IgnoredAny;

/// The method allows you to add an external PDF file to an invoice previously issued from BaseLinker.
/// It enables replacing a standard invoice from BaseLinker with an invoice issued e.g. in an ERP program.
#[derive(Serialize, Deserialize)]
pub struct AddOrderInvoiceFile {
    /// BaseLinker invoice identifier
    pub invoice_id: i64,
    /// Invoice PDF file in binary format encoded in base64, at the very beginning of the invoice string provide a prefix "data:" e.g. "data:4AAQSkSzkJRgABA[...]"
    pub file: String,
    /// External system invoice number (overwrites BaseLinker invoice number)
    pub external_invoice_number: String,
}

impl RequestTrait<IgnoredAny> for AddOrderInvoiceFile { const METHOD: &'static str = "addOrderInvoiceFile"; }