use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub bind: String,
    pub timezone: String,
    pub family_name: String,
    pub refresh_minutes: u64,
    pub chrome_path: String,
    pub icloud: IcloudConfig,
    pub sources: SourcesConfig,
    #[serde(skip)]
    pub config_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct IcloudConfig {
    pub apple_id: String,
    pub app_password: String,
    pub calendars: Vec<String>,
    pub todo_list: String,
    pub shopping_list: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SourcesConfig {
    pub ics_urls: Vec<String>,
    pub shopping_file: String,
    pub todos_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8765".into(),
            timezone: "Europe/London".into(),
            family_name: "Family".into(),
            refresh_minutes: 60,
            chrome_path: String::new(),
            icloud: IcloudConfig::default(),
            sources: SourcesConfig::default(),
            config_dir: PathBuf::from("."),
        }
    }
}

impl Default for IcloudConfig {
    fn default() -> Self {
        Self {
            apple_id: String::new(),
            app_password: String::new(),
            calendars: vec!["Family".into()],
            todo_list: "Family".into(),
            shopping_list: "Shopping".into(),
        }
    }
}

impl Default for SourcesConfig {
    fn default() -> Self {
        Self {
            ics_urls: Vec::new(),
            shopping_file: "fixtures/shopping.json".into(),
            todos_file: "fixtures/todos.json".into(),
        }
    }
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading config {}", path.display()))?;
        let mut cfg: Config = toml::from_str(&text).context("parsing config.toml")?;
        cfg.config_dir = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        Ok(cfg)
    }

    pub fn load_or_default(path: Option<&Path>) -> Result<Self> {
        match path {
            Some(p) => Self::load(p),
            None => {
                for candidate in [
                    Path::new("config.toml"),
                    Path::new("projects/eink-frame/server/config.toml"),
                ] {
                    if candidate.exists() {
                        return Self::load(candidate);
                    }
                }
                let mut cfg = Config::default();
                cfg.config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                Ok(cfg)
            }
        }
    }

    pub fn icloud_enabled(&self) -> bool {
        !self.icloud.apple_id.trim().is_empty() && !self.icloud.app_password.trim().is_empty()
    }

    pub fn resolve(&self, relative: &str) -> PathBuf {
        let p = Path::new(relative);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.config_dir.join(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_config_parses() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config.example.toml");
        let cfg = Config::load(path).unwrap();
        assert_eq!(cfg.family_name, "Family");
        assert_eq!(cfg.refresh_minutes, 60);
        assert_eq!(cfg.icloud.shopping_list, "Shopping");
    }
}
