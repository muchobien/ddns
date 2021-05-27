use std::str::FromStr;

use crate::providers::{
    cloudflare::{Cloudflare, CloudflareConfig},
    vercel::Vercel,
    Provider, Providers,
};
use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    host: String,
    provider: String,
    pub cloudflare: Option<CloudflareConfig>,
}

impl Settings {
    pub fn new() -> eyre::Result<Self> {
        let mut s = Config::default();
        s.merge(Environment::with_prefix("ddns").separator("_"))?;

        s.try_into().map_err(|err| eyre::eyre!(err))
    }

    pub fn provider(&self) -> eyre::Result<Box<dyn Provider>> {
        match Providers::from_str(&self.provider)
            .map_err(|_| eyre::eyre!("Provider not available"))?
        {
            Providers::Cloudflare => {
                Ok(Box::new(Cloudflare::new(self.cloudflare.clone().unwrap())?))
            }
            Providers::Vercel => Ok(Box::new(Vercel)),
        }
    }
}
