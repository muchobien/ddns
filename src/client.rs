use std::time::Duration;

use serde::Serialize;

use crate::framework::{
    async_api::{ApiClient, ApiResponse, ApiResult},
    auth::{self, AuthClient},
    endpoint::Endpoint,
    reqwest_adaptors::match_reqwest_method,
    Environment,
};

use async_trait::async_trait;

pub struct Client {
    environment: Environment,
    credentials: auth::Credentials,
    http_client: reqwest::Client,
}

pub struct HttpApiClientConfig {
    /// The maximum time limit for an API request. If a request takes longer than this, it will be
    /// cancelled.
    /// Note: this configuration has no effect when the target is wasm32.
    pub http_timeout: Duration,
    /// A default set of HTTP headers which will be sent with each API request.
    pub default_headers: http::HeaderMap,
}

impl Default for HttpApiClientConfig {
    fn default() -> Self {
        HttpApiClientConfig {
            http_timeout: Duration::from_secs(30),
            default_headers: http::HeaderMap::default(),
        }
    }
}

impl Client {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> eyre::Result<Client> {
        let builder = reqwest::Client::builder().default_headers(config.default_headers);

        let http_client = builder.build()?;

        Ok(Client {
            environment,
            credentials,
            http_client,
        })
    }

    pub async fn request_handle<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(
                match_reqwest_method(endpoint.method()),
                endpoint.url(&self.environment),
            )
            .query(&endpoint.query());

        if let Some(body) = endpoint.body() {
            request = request.body(serde_json::to_string(&body).unwrap());
            request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.auth(&self.credentials);
        let res = request.send().await?;
        if res.status().is_success() {
            return res.json::<ResultType>().await;
        }
        Err(res.error_for_status().unwrap_err())
    }
}

#[async_trait]
impl ApiClient for Client {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        self.request_handle(endpoint).await
    }
}

// async fn map_api_response<ResultType: ApiResult>(
//     resp: reqwest::Response,
// ) -> ApiResponse<ResultType> {
//     let status = resp.status();
//     if status.is_success() {
//         let parsed: Result<ResultType, reqwest::Error> = resp.json().await;
//         match parsed {
//             Ok(api_resp) => Ok(api_resp),
//             Err(e) => eyre!("{:?}", e),
//         }
//     }

//     eyre!("Not Allowed")
// }
