use super::auth::{AuthClient, Credentials};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub trait ApiResult: DeserializeOwned + Debug {}
impl ApiResult for () {}

pub type ApiResponse<ResultType> = eyre::Result<ResultType, reqwest::Error>;

impl AuthClient for reqwest::RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}
