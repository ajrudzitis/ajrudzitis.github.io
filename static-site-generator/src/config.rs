use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub base_url: String,
    pub author: String,

    #[serde(default)]
    pub social: SocialLinks,

    #[serde(default)]
    pub permalinks: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialLinks {
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
}

impl Config {
    pub fn load(site_dir: &Path) -> Result<Self> {
        let config_path = site_dir.join("config.toml");
        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse config.toml")?;

        Ok(config)
    }

    pub fn get_permalink_pattern(&self, category: &str) -> String {
        self.permalinks
            .get(category)
            .cloned()
            .unwrap_or_else(|| format!("/{}/:year-:month-:day", category))
    }
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub site_dir: PathBuf,
    pub content_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub static_dir: PathBuf,
    pub data_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl Paths {
    pub fn new(site_dir: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            content_dir: site_dir.join("content"),
            templates_dir: site_dir.join("templates"),
            static_dir: site_dir.join("static"),
            data_dir: site_dir.join("data"),
            site_dir,
            output_dir,
        }
    }
}
