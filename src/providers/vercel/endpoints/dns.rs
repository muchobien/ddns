use super::Pagination;
use crate::framework::{
    endpoint::{Endpoint, Method},
    async_api::ApiResult,
};
use serde::{Deserialize, Serialize};

/// List DNS Records
/// https://vercel.com/docs/api#endpoints/dns/list-all-the-dns-records-of-a-domain
#[derive(Debug)]
pub struct ListDnsRecords<'a> {
    pub domain: &'a str,
    pub params: ListDnsRecordsParams,
}

impl<'a> Endpoint<ListDnsRecordsResponse, ListDnsRecordsParams> for ListDnsRecords<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> String {
        format!("v4/domains/{}/records", self.domain)
    }

    fn query(&self) -> Option<ListDnsRecordsParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListDnsRecordsParams {
    pub limit: Option<u32>,
    pub since: Option<u64>,
    pub until: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDnsRecordsResponse {
    pub records: Vec<Record>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub creator: Option<String>,
    pub created: Option<String>,
    pub created_at: Option<String>,
    pub updated: Option<String>,
    pub updated_at: Option<String>,
}

impl ApiResult for ListDnsRecordsResponse {}
