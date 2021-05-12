use serde::Deserialize;
pub mod dns;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub count: u64,
    pub next: u64,
    pub prev: u64,
}
