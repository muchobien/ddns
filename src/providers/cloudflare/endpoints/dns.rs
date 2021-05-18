use super::{OrderDirection, Response, SearchMatch};
use crate::framework::{
    async_api::ApiResult,
    endpoint::{Endpoint, Method},
};
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

/// List DNS Records
/// https://vercel.com/docs/api#endpoints/dns/list-all-the-dns-records-of-a-domain
#[derive(Debug)]
pub struct ListDnsRecords<'a> {
    pub zone_identifier: &'a str,
    pub params: ListDnsRecordsParams,
}

impl<'a> Endpoint<Response<Vec<Record>>, ListDnsRecordsParams> for ListDnsRecords<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> String {
        format!("zones/{}/dns_records", self.zone_identifier)
    }

    fn query(&self) -> Option<ListDnsRecordsParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListDnsRecordsOrder {
    Type,
    Name,
    Content,
    Ttl,
    Proxied,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListDnsRecordsParams {
    pub record_type: Option<DnsContent>,
    pub name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListDnsRecordsOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    /// Will exist if Cloudflare automatically added this DNS record during initial setup.
    pub auto_added: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum DnsContent {
    A { content: Ipv4Addr },
    AAAA { content: Ipv6Addr },
    CNAME { content: String },
    NS { content: String },
    MX { content: String, priority: u16 },
    TXT { content: String },
    SRV { content: String },
}

#[derive(Deserialize, Debug)]
pub struct Record {
    /// Extra Cloudflare-specific information about the record
    pub meta: Meta,
    /// Whether this record can be modified/deleted (true means it's managed by Cloudflare)
    pub locked: bool,
    /// DNS record name
    pub name: String,
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: u32,
    /// Zone identifier tag
    pub zone_id: String,
    /// When the record was last modified
    pub modified_on: DateTime<Utc>,
    /// When the record was created
    pub created_on: DateTime<Utc>,
    /// Whether this record can be modified/deleted (true means it's managed by Cloudflare)
    pub proxiable: bool,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DnsContent,
    /// DNS record identifier tag
    pub id: String,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: bool,
    /// The domain of the record
    pub zone_name: String,
}

impl ApiResult for Response<Vec<Record>> {}
