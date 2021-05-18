use ddns::config::Config;
use ddns::framework::async_api::ApiClient;
use ddns::{
    client::{Client, HttpApiClientConfig},
    providers::cloudflare::endpoints::dns,
};
use eyre::Result;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
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
