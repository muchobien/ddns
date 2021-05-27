use super::Provider;
use async_trait::async_trait;

pub struct Vercel;

#[async_trait]
impl Provider for Vercel {
    async fn update(&self, _ip: std::net::IpAddr) -> eyre::Result<()> {
        Ok(())
    }
}
