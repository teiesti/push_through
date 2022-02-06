use {serde::Deserialize, std::path::PathBuf};

#[derive(Debug, Deserialize)]
pub(crate) struct Deployment {
    hook: String,
    repo: String,
    key: Option<PathBuf>,
    path: PathBuf,
}
