mod client;
mod config;
mod framework;
mod providers;

use structopt::StructOpt;

use crate::{
    client::{Client, HttpApiClientConfig},
    config::Config,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = Config::from_args();
    let client = Client::new(
        config.credentials(),
        HttpApiClientConfig::default(),
        config.provider(),
    )?;

    let ip = public_ip::addr()
        .await
        .ok_or(eyre::eyre!("Unable to get public ip"))?;

    let updater = config.updater();

    updater.update(ip, &config, &client).await
}
