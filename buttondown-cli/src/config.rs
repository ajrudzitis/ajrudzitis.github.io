use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Default API key file name
pub const DEFAULT_API_KEY_FILE: &str = ".buttondown-api-key";

/// Configuration for the Buttondown CLI
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub letters_dir: PathBuf,
    pub dry_run: bool,
    pub verbose: bool,
}

impl Config {
    /// Load configuration from file, environment, and CLI args
    ///
    /// API key lookup order:
    /// 1. File specified by `api_key_file` parameter
    /// 2. Default file `.buttondown-api-key` in current directory
    /// 3. `BUTTONDOWN_API_KEY` environment variable
    pub fn load(
        api_key_file: Option<PathBuf>,
        letters_dir: Option<PathBuf>,
        dry_run: bool,
        verbose: bool,
    ) -> Result<Self> {
        let api_key = load_api_key(api_key_file)?;

        let letters_dir = letters_dir.unwrap_or_else(|| {
            PathBuf::from("site/content/posts/letters")
        });

        Ok(Self {
            api_key,
            letters_dir,
            dry_run,
            verbose,
        })
    }

    /// Load config but don't require API key (for local-only operations)
    pub fn load_without_api(letters_dir: Option<PathBuf>, dry_run: bool, verbose: bool) -> Self {
        let api_key = load_api_key(None).unwrap_or_default();

        let letters_dir = letters_dir.unwrap_or_else(|| {
            PathBuf::from("site/content/posts/letters")
        });

        Self {
            api_key,
            letters_dir,
            dry_run,
            verbose,
        }
    }
}

/// Load API key from file or environment variable
fn load_api_key(api_key_file: Option<PathBuf>) -> Result<String> {
    // Try specified file first
    if let Some(ref path) = api_key_file {
        return read_api_key_file(path)
            .with_context(|| format!("Failed to read API key from {:?}", path));
    }

    // Try default file
    let default_path = PathBuf::from(DEFAULT_API_KEY_FILE);
    if default_path.exists() {
        return read_api_key_file(&default_path)
            .with_context(|| format!("Failed to read API key from {:?}", default_path));
    }

    // Fall back to environment variable
    std::env::var("BUTTONDOWN_API_KEY").with_context(|| {
        format!(
            "API key not found. Please either:\n  \
             - Create a file named '{}' containing your API key\n  \
             - Set the BUTTONDOWN_API_KEY environment variable",
            DEFAULT_API_KEY_FILE
        )
    })
}

/// Read and trim API key from file
fn read_api_key_file(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path)?;
    let key = content.trim().to_string();

    if key.is_empty() {
        anyhow::bail!("API key file is empty");
    }

    Ok(key)
}

/// API constants
pub const BUTTONDOWN_API_BASE: &str = "https://api.buttondown.email/v1";
