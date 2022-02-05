use {
    anyhow::{bail, Context, Result},
    const_format::formatcp,
    log::{info, trace},
    serde::Deserialize,
    std::{
        collections::HashMap,
        env,
        fs::read_to_string,
        net::SocketAddr,
        ops::Deref,
        path::{Path, PathBuf},
    },
};

const SEARCH_PATHS: &[&str] = &[
    "./",
    formatcp!("~/.config/{}/", crate::PKG_NAME),
    formatcp!("/etc/{}/", crate::PKG_NAME),
];
const FILE_NAME: &str = formatcp!("{}.toml", crate::PKG_NAME);

#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    socket: SocketAddr,
    deployments: Vec<Deployment>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Deployment {
    repository: String,
    key: Option<PathBuf>,
    path: PathBuf,
    hook: String,
}

impl Configuration {
    pub(crate) fn discover() -> Result<Self> {
        info!("Searching for a configuration file");

        let manifest_dir = env::var("CARGO_MANIFEST_DIR");
        let mut paths = manifest_dir
            .iter()
            .map(String::as_str)
            .chain(SEARCH_PATHS.iter().map(Deref::deref))
            .map(Path::new)
            .map(|directory| directory.join(FILE_NAME));

        let path = loop {
            match paths.next() {
                Some(path) => {
                    trace!("Trying {}", path.display());
                    if path.exists() {
                        break path;
                    }
                }
                None => bail!("Could not find a configuration file"),
            }
        };

        Self::load(path)
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("Loading configuration from {}", path.display());

        let string =
            read_to_string(path).with_context(|| format!("Could not read {}", path.display()))?;

        let config = toml::from_str(&string)
            .with_context(|| format!("Could not decode {}", path.display()))?;

        Ok(config)
    }

    pub(crate) fn socket(&self) -> &SocketAddr {
        &self.socket
    }

    pub(crate) fn hooked_deployments(&self) -> Result<HashMap<String, Deployment>> {
        let mut result = HashMap::with_capacity(self.deployments.len());
        for deployment in &self.deployments {
            let hook = deployment.hook.clone();
            if let Some(_) = result.insert(hook, deployment.clone()) {
                bail!("The hook {} is not unique", deployment.hook);
            }
        }
        Ok(result)
    }
}
