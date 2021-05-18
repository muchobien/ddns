pub mod endpoints;

use crate::framework::Environment;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENVIRONMENT: Environment =
        Environment::Custom(url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap());
}
