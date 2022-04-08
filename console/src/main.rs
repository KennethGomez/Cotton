use essentials::config::{get_config, ConfigurationProfile};
use essentials::logger;

fn main() {
    // Initialize environment variables from .env file to be accessible from other packages
    // with the std function std::env()
    dotenv::dotenv().ok();
    logger::init(log::LevelFilter::Trace);

    log::info!("reading configuration...");

    let config = get_config(ConfigurationProfile::Environment).unwrap();

    log::info!("initializing server...");

    let _ = server::initialize(&config).unwrap();
}
