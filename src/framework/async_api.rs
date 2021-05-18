use super::{
    auth::{AuthClient, Credentials},
    endpoint::Endpoint,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait ApiResult: DeserializeOwned + Debug {}
impl ApiResult for () {}

pub type ApiResponse<ResultType> = eyre::Result<ResultType, reqwest::Error>;

#[async_trait]
pub trait ApiClient {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;
}

impl AuthClient for reqwest::RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}
