mod config;
mod deploy;

use {
    crate::config::Configuration,
    anyhow::{Context, Error, Result},
    env_logger::Env,
    log::{error, info},
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_COMMIT: &str = env!("PKG_COMMIT");

#[rocket::main]
async fn main() {
    if let Err(err) = try_main().await {
        handle_error(err);
    }
}

async fn try_main() -> Result<()> {
    // Initialize logging
    let env = Env::default().default_filter_or(format!("{}=info", PKG_NAME));
    env_logger::init_from_env(env);

    // Log the version
    info!("Starting {} {} ({})", PKG_NAME, PKG_VERSION, PKG_COMMIT);

    // Load the configuration
    let configuration = Configuration::discover()?;

    // Start the HTTP server
    info!(
        "Starting the HTTP server at http://{}",
        configuration.socket
    );
    rocket::custom(rocket::Config {
        address: configuration.socket.ip(),
        port: configuration.socket.port(),
        ..rocket::Config::default()
    })
    .mount("/", configuration.into_routes())
    .launch()
    .await
    .context("Error while running the HTTP server")?;

    Ok(())
}

fn handle_error(err: Error) {
    error!("{:?}", err);
}
