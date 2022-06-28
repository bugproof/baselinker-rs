use crate::common::{BaseLinkerError, Error, RequestTrait};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::result::Result::Err;
use std::time::Duration;
use tower::limit::RateLimit;
use tower::{Service, ServiceBuilder};

#[derive(Deserialize, Debug)]
struct BaseLinkerResponse {
    pub status: String,

    #[serde(flatten)]
    pub error: Option<BaseLinkerError>,
}

pub struct BaseLinkerClient {
    token: String,
    rate_limit: RateLimit<reqwest::Client>,
}

impl BaseLinkerClient {
    pub fn new(token: String, http_client: reqwest::Client) -> Self {
        Self {
            token,
            rate_limit: ServiceBuilder::new()
                .rate_limit(100, Duration::from_secs(60))
                .service(http_client),
        }
    }

    pub async fn send<Request, Response>(&mut self, request: &Request) -> Result<Response, Error>
    where
        Request: RequestTrait<Response> + Serialize,
        Response: DeserializeOwned,
    {
        let parameters = serde_json::to_string(request).unwrap();

        let mut params = HashMap::new();
        params.insert("method", Request::METHOD);
        params.insert("parameters", parameters.as_str());

        let http_request = self
            .rate_limit
            .get_ref()
            .post("https://api.baselinker.com/connector.php")
            .header("X-BLToken", self.token.as_str())
            .form(&params)
            .build()?;

        let response = self.rate_limit.call(http_request).await?;
        let text = response.text().await.unwrap();

        let api_response = serde_json::from_str::<BaseLinkerResponse>(text.as_str()).unwrap();
        if api_response.status != "SUCCESS" {
            return Err(Error::BaseLinkerError(api_response.error.unwrap()));
        }

        return Ok(serde_json::from_str(text.as_str()).unwrap());
    }
}
