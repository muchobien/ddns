use super::Provider;
use async_trait::async_trait;
use cloudflare::endpoints::{
    dns::{self, DnsRecord, DnsRecordType, Meta},
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

    async fn should_update<'a>(
        &self,
        zone_identifier: &'a str,
        ip: IpAddr,
    ) -> eyre::Result<Option<DnsRecord>> {
        let res = self
            .client
            .request(&dns::ListDnsRecords {
                zone_identifier,
                params: dns::ListDnsRecordsParams {
                    record_type: Some(match ip {
                        IpAddr::V4(_) => DnsRecordType::A,
                        IpAddr::V6(_) => DnsRecordType::AAAA,
                    }),
                    direction: Some(OrderDirection::Ascending),
                    name: Some(format!(
                        "{}.{}",
                        self.config.host.clone(),
                        self.config.domain.clone()
                    )),
                    per_page: Some(1),
                    ..Default::default()
                },
            })
            .await?;

        let record = &res.result[0];

        let is_updated = match record.content {
            dns::DnsContent::A { content } => content == ip,
            dns::DnsContent::AAAA { content } => content == ip,
            _ => false,
        };

        Ok(match is_updated {
            true => None,
            false => Some(DnsRecord {
                meta: Meta {
                    auto_added: record.meta.auto_added.clone(),
                },
                locked: record.locked,
                name: record.name.clone(),
                ttl: record.ttl,
                zone_id: record.zone_id.clone(),
                modified_on: record.modified_on,
                created_on: record.created_on,
                proxiable: record.proxiable,
                content: record.content.clone(),
                id: record.id.clone(),
                proxied: record.proxied,
                zone_name: record.zone_name.clone(),
            }),
        })
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

        if !is_updated {
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
            println!("Dns actualizado")
        } else {
            println!("Dns no actualizado")
        }

        Ok(())
    }
}
