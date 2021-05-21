use crate::{client::Client, config::Config, providers::vercel::endpoints::dns};

use super::Updater;
use async_trait::async_trait;
use std::net::IpAddr;

pub mod endpoints;

pub struct Vercel;

#[async_trait]
impl Updater for Vercel {
    async fn update(&self, ip: IpAddr, config: &Config, client: &Client) -> eyre::Result<()> {
        let res = client
            .request(&dns::ListDnsRecords {
                domain: config.zone.clone().unwrap().as_ref(),
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
