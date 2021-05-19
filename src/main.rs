mod client;
mod config;
mod framework;
mod providers;

use structopt::StructOpt;

use crate::{
    client::{Client, HttpApiClientConfig},
    config::Config,
    framework::async_api::ApiClient,
    providers::cloudflare::endpoints::dns,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = Config::from_args();
    let client = Client::new(
        config.credentials(),
        HttpApiClientConfig::default(),
        config.provider(),
    )?;

    let res = client
        .request(&dns::ListDnsRecords {
            zone_identifier: &config.zone.clone().unwrap(),
            params: dns::ListDnsRecordsParams {
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", res);
    Ok(())
}
