use structopt::clap::{arg_enum, AppSettings};
use structopt::StructOpt;
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
    host: String,
    #[structopt(short, long, possible_values=&Providers::variants(), case_insensitive= true, env = "DDNS_PROVIDER")]
    provider: String,
    #[structopt(short, long, env = "DDNS_PROVIDER_KEY")]
    key: String,
    #[structopt(short, long, possible_values=&IpKind::variants(), default_value="v4", case_insensitive= true, env = "DDNS_IP_KIND")]
    ip_kind: String,
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
        Cloudflare
    }
}
