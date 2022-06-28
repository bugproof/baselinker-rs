use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesNumberingInfo {
    pub id: i64,
    pub r#type: String,
    pub name: String,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSeriesResponse {
    pub series: Vec<SeriesNumberingInfo>,
}

/// The method allows to download a series of invoice/receipt numbering.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSeries {}

impl RequestTrait<GetSeriesResponse> for GetSeries {
    const METHOD: &'static str = "getSeries";
}
