use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize)]
pub struct SeriesNumberingInfo {
    pub id: i64,
    pub r#type: String,
    pub name: String,
    pub format: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetSeriesResponse {
    pub series: Vec<SeriesNumberingInfo>,
}

/// The method allows to download a series of invoice/receipt numbering.
#[derive(Serialize, Deserialize)]
pub struct GetSeries {}

impl RequestTrait<GetSeriesResponse> for GetSeries { const METHOD: &'static str = "getSeries"; }