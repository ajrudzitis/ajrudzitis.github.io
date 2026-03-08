use anyhow::{Context, Result};
use chrono::NaiveDate;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::models::{Frontmatter, LocalLetter};

/// Load all letters from the letters directory
pub fn load_letters(letters_dir: &Path) -> Result<Vec<LocalLetter>> {
    let mut letters = Vec::new();

    if !letters_dir.exists() {
        return Ok(letters);
    }

    for entry in WalkDir::new(letters_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str());

        if ext == Some("md") || ext == Some("html") {
            match LocalLetter::from_file(path) {
                Ok(letter) => letters.push(letter),
                Err(e) => eprintln!("Warning: Failed to parse {:?}: {}", path, e),
            }
        }
    }

    // Sort by date (newest first)
    letters.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(letters)
}

impl LocalLetter {
    /// Parse a letter from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        let is_html = path.extension().is_some_and(|ext| ext == "html");

        let (frontmatter, body) = parse_frontmatter(&content, is_html)?;

        // For HTML files, body is already HTML; for markdown, we'd convert
        // For now, we keep the raw body for API submission
        let html = if is_html {
            body.clone()
        } else {
            // Simple markdown to HTML - in real usage, you might want pulldown-cmark
            body.clone()
        };

        // Extract date and slug from filename
        let (date, slug) = extract_date_and_slug(path)?;

        Ok(Self {
            path: path.to_path_buf(),
            title: frontmatter.title.clone(),
            body,
            html,
            date,
            slug,
            buttondown_id: frontmatter.buttondown_id.clone(),
            frontmatter,
        })
    }

    /// Update the frontmatter with a buttondown_id and write back to file
    pub fn write_buttondown_id(&mut self, buttondown_id: &str) -> Result<()> {
        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read file: {:?}", self.path))?;

        let is_html = self.path.extension().is_some_and(|ext| ext == "html");

        // Update frontmatter
        self.frontmatter.buttondown_id = Some(buttondown_id.to_string());
        self.buttondown_id = Some(buttondown_id.to_string());

        let new_content = if has_frontmatter(&content) {
            // Replace existing frontmatter
            update_frontmatter(&content, &self.frontmatter)?
        } else {
            // Add frontmatter to file without it
            add_frontmatter(&content, &self.frontmatter, is_html)?
        };

        fs::write(&self.path, new_content)
            .with_context(|| format!("Failed to write file: {:?}", self.path))?;

        Ok(())
    }
}

fn parse_frontmatter(content: &str, is_html: bool) -> Result<(Frontmatter, String)> {
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap();

    if let Some(caps) = re.captures(content) {
        let yaml = &caps[1];
        let body = caps[2].to_string();

        let frontmatter: Frontmatter = serde_yaml::from_str(yaml)
            .with_context(|| "Failed to parse YAML frontmatter")?;

        Ok((frontmatter, body))
    } else {
        // No frontmatter, try to extract title from HTML <title> tag
        let title = if is_html {
            extract_html_title(content).unwrap_or_else(|| "Untitled".to_string())
        } else {
            "Untitled".to_string()
        };

        Ok((
            Frontmatter {
                title,
                layout: None,
                date: None,
                buttondown_id: None,
                extra: HashMap::new(),
            },
            content.to_string(),
        ))
    }
}

fn extract_html_title(html: &str) -> Option<String> {
    let title_re = Regex::new(r"(?i)<title>([^<]+)</title>").unwrap();
    title_re.captures(html).map(|caps| caps[1].trim().to_string())
}

fn extract_date_and_slug(path: &Path) -> Result<(Option<NaiveDate>, String)> {
    let filename = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled");

    let date_re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})-(.+)$").unwrap();

    if let Some(caps) = date_re.captures(filename) {
        let year: i32 = caps[1].parse()?;
        let month: u32 = caps[2].parse()?;
        let day: u32 = caps[3].parse()?;
        let slug = caps[4].to_string();

        let date = NaiveDate::from_ymd_opt(year, month, day)
            .with_context(|| format!("Invalid date: {}-{}-{}", year, month, day))?;

        Ok((Some(date), slug))
    } else {
        Ok((None, filename.to_string()))
    }
}

fn has_frontmatter(content: &str) -> bool {
    content.starts_with("---")
}

fn update_frontmatter(content: &str, frontmatter: &Frontmatter) -> Result<String> {
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap();

    if let Some(caps) = re.captures(content) {
        let body = &caps[2];
        let yaml = serde_yaml::to_string(frontmatter)
            .with_context(|| "Failed to serialize frontmatter")?;

        Ok(format!("---\n{}---\n{}", yaml, body))
    } else {
        anyhow::bail!("Content has no frontmatter to update")
    }
}

fn add_frontmatter(content: &str, frontmatter: &Frontmatter, _is_html: bool) -> Result<String> {
    let yaml = serde_yaml::to_string(frontmatter)
        .with_context(|| "Failed to serialize frontmatter")?;

    Ok(format!("---\n{}---\n{}", yaml, content))
}

/// Normalize a string for comparison (lowercase, remove punctuation)
pub fn normalize_for_comparison(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
