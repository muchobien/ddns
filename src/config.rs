use structopt::clap::{arg_enum, AppSettings};
use structopt::StructOpt;

use crate::{
    framework::{auth::Credentials, Environment},
    providers::{self, Updater},
};
#[derive(Debug, StructOpt)]
#[structopt(
    about,
    setting(AppSettings::ColoredHelp),
    setting(AppSettings::ColorAuto)
)]
pub struct Config {
    #[structopt(short, long, env = "DDNS_DOMAIN")]
    pub domain: String,
    #[structopt(short, long, env = "DDNS_HOST")]
    pub host: String,
    #[structopt(short, long, case_insensitive = true, env = "DDNS_PROVIDER", possible_values = &Providers::variants())]
    provider: Providers,
    #[structopt(short, long, env = "DDNS_PROVIDER_KEY")]
    key: String,
    #[structopt(
        short,
        long,
        default_value = "v4",
        case_insensitive = true,
        env = "DDNS_IP_KIND",
        possible_values = &IpKind::variants()
    )]
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
        Vercel,
    }
}

impl Config {
    pub fn provider(&self) -> Environment {
        match self.provider {
            Providers::Cloudflare => {
                Environment::Base(url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap())
            }
            Providers::Vercel => {
                Environment::Base(url::Url::parse("https://api.vercel.com/").unwrap())
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

    pub fn updater(&self) -> Box<dyn Updater> {
        match self.provider {
            Providers::Cloudflare => Box::new(providers::Cloudflare),
            Providers::Vercel => Box::new(providers::Vercel),
        }
    }
}
