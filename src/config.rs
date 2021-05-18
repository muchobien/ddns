use structopt::clap::{arg_enum, AppSettings};
use structopt::StructOpt;

use crate::framework::{auth::Credentials, Environment};
#[derive(Debug, StructOpt)]
#[structopt(
    about,
    setting(AppSettings::ColoredHelp),
    setting(AppSettings::ColorAuto)
)]
pub struct Config {
    #[structopt(short, long, env = "DDNS_DOMAIN")]
    domain: String,
    #[structopt(short, long, env = "DDNS_HOST")]
    pub host: String,
    #[structopt(short, long, possible_values=&Providers::variants(), case_insensitive= true, env = "DDNS_PROVIDER")]
    provider: Providers,
    #[structopt(short, long, env = "DDNS_PROVIDER_KEY")]
    key: String,
    #[structopt(short, long, possible_values=&IpKind::variants(), default_value="v4", case_insensitive= true, env = "DDNS_IP_KIND")]
    ip_kind: String,
    #[structopt(
        short,
        long,
        env = "DDNS_PROVIDER_EMAIL",
        required_if("provider", "Cloudflare")
    )]
    email: Option<String>,
    #[structopt(
        short,
        long,
        env = "DDNS_PROVIDER_ZONE",
        required_if("provider", "Cloudflare")
    )]
    pub zone: Option<String>,
}
arg_enum! {
    #[derive(Debug, PartialEq)]
    enum IpKind {
        V4,
        V6,
    }
}

arg_enum! {
    #[derive(Debug, PartialEq)]
    enum Providers {
        Cloudflare,
        Vercel
    }
}

impl Config {
    pub fn provider(&self) -> Environment {
        match self.provider {
            Providers::Cloudflare => Environment::Custom(
                url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap(),
            ),
            Providers::Vercel => {
                Environment::Custom(url::Url::parse("https://api.vercel.com/").unwrap())
            }
        }
    }

    pub fn credentials(&self) -> Credentials {
        match self.provider {
            Providers::Cloudflare => Credentials::UserAuthKey {
                email: self.email.clone().unwrap(),
                key: self.key.clone(),
            },
            Providers::Vercel => Credentials::UserAuthToken {
                token: self.key.clone(),
            },
        }
    }
}
