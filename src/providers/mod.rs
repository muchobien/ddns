use std::net::IpAddr;

use crate::{client::Client, config::Config};
use async_trait::async_trait;

pub(crate) mod cloudflare;
pub(crate) mod vercel;
pub use cloudflare::Cloudflare;
pub use vercel::Vercel;

#[async_trait]
pub trait Updater {
    async fn update(&self, ip: IpAddr, config: &Config, client: &Client) -> eyre::Result<()>;
}
