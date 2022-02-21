use {
    crate::*,
    anyhow::*,
    lazy_regex::regex_is_match,
    serde::Deserialize,
    std::{collections::HashMap, fs, path::Path},
};

/// the configuration item which may be stored as `baking.toml`
/// along a `Cargo.toml` file
#[derive(Debug, Clone, Deserialize)]
pub struct PackageConfig {
    pub default_job: String,
    pub jobs: HashMap<String, Job>,
    pub keybindings: Option<KeyBindings>,
}

impl PackageConfig {
    pub fn from_path(path: &Path) -> Result<Self> {
        let conf = toml::from_str::<PackageConfig>(&fs::read_to_string(path)?)
            .with_context(|| format!("Failed to parse configuration file at {:?}", path))?;
        if conf.jobs.is_empty() {
            bail!("Invalid baking.toml : no job found");
        }
        for (name, job) in &conf.jobs {
            if !regex_is_match!(r#"^[\w-]+$"#, name) {
                bail!(
                    "Invalid baking.toml : Illegal job name : {:?}",
                    name
                );
            }
            if job.command.is_empty() {
                bail!(
                    "Invalid baking.toml : empty command for job {:?}",
                    name
                );
            }
        }
        if !conf.jobs.contains_key(&conf.default_job) {
            bail!(
                "Invalid baking.toml : default job not found in jobs"
            );
        }
        Ok(conf)
    }
}

impl Default for PackageConfig {
    fn default() -> Self {
        toml::from_str(DEFAULT_PACKAGE_CONFIG).unwrap()
    }
}
