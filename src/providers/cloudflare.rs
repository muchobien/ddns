use super::Provider;
use async_trait::async_trait;
use cloudflare::endpoints::{
    dns::{self, DnsRecordType},
    zone::{self, Status},
};
use serde::Deserialize;
use std::net::IpAddr;

use cloudflare::framework::{
    async_api::{ApiClient, Client},
    auth::Credentials,
    Environment, HttpApiClientConfig, OrderDirection,
};

#[derive(Debug, Deserialize, Clone)]
pub struct CloudflareConfig {
    domain: String,
    key: String,
    email: String,
    host: String,
}

pub struct Cloudflare {
    client: Client,
    config: CloudflareConfig,
}

impl Cloudflare {
    pub fn new(config: CloudflareConfig) -> eyre::Result<Self> {
        let client = Client::new(
            Credentials::UserAuthKey {
                key: config.key.clone(),
                email: config.email.clone(),
            },
            HttpApiClientConfig::default(),
            Environment::Production,
        )
        .map_err(|err| eyre::eyre!(err))?;

        Ok(Self { client, config })
    }

    async fn zone(&self) -> eyre::Result<String> {
        let res = self
            .client
            .request(&zone::ListZones {
                params: zone::ListZonesParams {
                    name: Some(self.config.domain.clone()),
                    status: Some(Status::Active),
                    per_page: Some(1),
                    ..Default::default()
                },
            })
            .await?;

        Ok(res.result[0].id.clone())
    }

    fn list_records<'a>(&self, ip: &IpAddr, zone_identifier: &'a str) -> dns::ListDnsRecords<'a> {
        dns::ListDnsRecords {
            zone_identifier,
            params: dns::ListDnsRecordsParams {
                direction: Some(OrderDirection::Ascending),
                name: Some(format!(
                    "{}.{}",
                    self.config.host.clone(),
                    self.config.domain.clone()
                )),
                per_page: Some(1),
                record_type: Some(match ip {
                    IpAddr::V4(_) => DnsRecordType::A,
                    IpAddr::V6(_) => DnsRecordType::AAAA,
                }),
                ..Default::default()
            },
        }
    }
}

#[async_trait]
impl Provider for Cloudflare {
    async fn update(&self, ip: IpAddr) -> eyre::Result<()> {
        let zone = self.zone().await?;
        let response = self.client.request(&self.list_records(&ip, &zone)).await?;

        let record = &response.result[0];

        let is_updated = match record.content {
            dns::DnsContent::A { content } => content == ip,
            dns::DnsContent::AAAA { content } => content == ip,
            _ => false,
        };

        match is_updated {
            true => {
                println!("DNS is updated!")
            }
            false => {
                self.client
                    .request(&dns::UpdateDnsRecord {
                        identifier: &record.id,
                        zone_identifier: &zone,
                        params: dns::UpdateDnsRecordParams {
                            ttl: None,
                            proxied: None,
                            name: &record.name,
                            content: match ip {
                                IpAddr::V4(content) => dns::DnsContent::A { content },
                                IpAddr::V6(content) => dns::DnsContent::AAAA { content },
                            },
                        },
                    })
                    .await?;
                println!("DNS updated!")
            }
        }

        Ok(())
    }
}
