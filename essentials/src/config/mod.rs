use std::env;
use std::net::IpAddr;
use std::str::FromStr;

use getset::{Getters, CopyGetters};
use anyhow::Result;

#[derive(Getters, Debug)]
pub struct CottonConfiguration {
    #[getset(get = "pub")]
    server: CottonServerConfiguration,
}

#[derive(CopyGetters, Debug)]
pub struct CottonServerConfiguration {
    #[getset(get_copy = "pub")]
    ip: IpAddr,

    #[getset(get_copy = "pub")]
    port: u16,
}

// The way of Cotton configuration to be retrieved
#[derive(Debug)]
pub enum ConfigurationProfile {
    // Meaning all the configuration variables are retrieved from the environment variables
    Environment,
}

pub fn get_config(profile: ConfigurationProfile) -> Result<CottonConfiguration> {
    match profile {
        ConfigurationProfile::Environment => config_from_env(),
    }
}

fn config_from_env() -> Result<CottonConfiguration> {
    Ok(CottonConfiguration {
        server: CottonServerConfiguration {
            ip: IpAddr::from_str(&env::var("SERVER_IP")?)?,
            port: env::var("SERVER_PORT")?.parse()?,
        },
    })
}
