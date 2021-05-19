pub mod dns;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

#[derive(Serialize, Clone, Debug)]
#[allow(dead_code)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Used as a parameter to API calls that search for a resource (e.g. DNS records).
/// Tells the API whether to return results that match all search requirements or at least one (any).
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum SearchMatch {
    /// Match all search requirements
    All,
    /// Match at least one search requirement
    Any,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response<ResultType> {
    pub result: ResultType,
    pub result_info: Option<JsonValue>,
    pub messages: JsonValue,
    pub errors: Vec<ResponseError>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub code: u16,
    pub message: String,
    #[serde(flatten)]
    pub other: HashMap<String, JsonValue>,
}

impl PartialEq for ResponseError {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.message == other.message
    }
}

impl Eq for ResponseError {}
