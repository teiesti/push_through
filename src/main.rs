mod config;
mod service;

use {
    crate::{config::Configuration, service::PushThrough},
    anyhow::{Context, Error, Result},
    env_logger::Env,
    hyper::{service::make_service_fn, Server},
    log::{error, info},
    std::convert::Infallible,
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_COMMIT: &str = env!("PKG_COMMIT");

#[tokio::main]
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
    info!("Starting the HTTP server");
    let server = Server::try_bind(configuration.socket())
        .with_context(|| format!("Could not bind to http://{}", configuration.socket()))?
        .serve(make_service_fn(|_| async {
            Ok::<_, Infallible>(PushThrough {})
        }));

    info!("Listening to http://{}", configuration.socket());
    server.await.context("Error while running the server")
}

fn handle_error(err: Error) {
    error!("{:?}", err);
}
