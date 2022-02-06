use {
    env_logger::Env,
    log::info,
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_COMMIT: &str = env!("PKG_COMMIT");

fn main() {
    // Initialize logging
    let env = Env::default().default_filter_or(format!("{}=info", PKG_NAME));
    env_logger::init_from_env(env);

    // Log the version
    info!("Starting {} {} ({})", PKG_NAME, PKG_VERSION, PKG_COMMIT);

    // TODO
}
