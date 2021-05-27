use async_trait::async_trait;
use serde::Deserialize;
use std::net::IpAddr;
use strum::{EnumString, EnumVariantNames};

pub mod cloudflare;
pub mod vercel;

#[async_trait]
pub trait Provider {
    async fn update(&self, ip: IpAddr) -> eyre::Result<()>;
}

#[derive(Debug, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Providers {
    Cloudflare,
    Vercel,
}
