use serde::{Deserialize, Serialize};

pub trait RequestTrait<T> {
    const METHOD: &'static str;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseLinkerError {
    #[serde(rename = "error_code")]
    pub code: String,

    #[serde(rename = "error_message")]
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("BaseLinker error")]
    BaseLinkerError(BaseLinkerError),

    #[error("Network error `{0}`")]
    NetworkError(#[from] reqwest::Error),
}
