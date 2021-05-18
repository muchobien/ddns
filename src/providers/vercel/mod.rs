use crate::framework::Environment;
pub mod endpoints;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENVIRONMENT: Environment =
        Environment::Custom(url::Url::parse("https://api.vercel.com/").unwrap());
}
