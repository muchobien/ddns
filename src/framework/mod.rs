pub mod async_api;
pub mod auth;
pub mod endpoint;
pub mod reqwest_adaptors;

#[derive(Debug)]
pub enum Environment {
    Base(url::Url),
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Base(url) => url.clone(),
        }
    }
}
