use std::time::Duration;

use crate::framework::{auth, Environment};

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
}
