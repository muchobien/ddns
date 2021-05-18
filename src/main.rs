use ddns::framework::async_api::ApiClient;
use ddns::framework::auth::Credentials;
use ddns::providers::cloudflare::endpoints::dns;
use ddns::{
    client::{Client, HttpApiClientConfig},
    framework::Environment,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(
        Credentials::UserAuthKey{
            email: "email".to_string(),
            key: "llave".to_string(),
        },
        HttpApiClientConfig::default(),
        Environment::Custom(url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap()),
    )?;
    let res = client
        .request(&dns::ListDnsRecords {
            zone_identifier: "zonita",
            params: dns::ListDnsRecordsParams {
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", res);
    Ok(())
}
