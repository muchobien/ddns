use std::net::IpAddr;

use crate::{client::Client, config::Config, providers::cloudflare::endpoints::dns};
use async_trait::async_trait;

use super::Updater;

pub mod endpoints;

pub struct Cloudflare;

#[async_trait]
impl Updater for Cloudflare {
    async fn update(&self, ip: IpAddr, config: &Config, client: &Client) -> eyre::Result<()> {
        let res = client
            .request(&dns::ListDnsRecords {
                zone_identifier: config.zone.clone().unwrap().as_ref(),
                params: dns::ListDnsRecordsParams {
                    ..Default::default()
                },
            })
            .await?;
        println!("{:?}", res);
        println!("{:?}", ip);
        Ok(())
    }
}
