use super::{
    auth::{AuthClient, Credentials},
    endpoint::Endpoint,
    response::{ApiResponse, ApiResult},
};
use async_trait::async_trait;
use serde::Serialize;

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
