use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub trait ApiResult: DeserializeOwned + Debug {}
