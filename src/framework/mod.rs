pub mod async_api;
pub mod auth;
pub mod endpoint;
pub mod reqwest_adaptors;

#[derive(Debug)]
pub enum Environment {
    Custom(url::Url),
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Custom(url) => url.clone(),
        }
    }
}
