use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use crate::config::{Config, Paths};
use crate::content::Content;
use crate::template::TemplateEngine;

#[derive(Debug, Serialize)]
struct PageContext {
    site: Config,
    page: Content,
    posts_by_category: HashMap<String, Vec<Content>>,
    data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct PostContext {
    site: Config,
    page: Content,
}

#[derive(Debug, Serialize)]
struct ListContext {
    site: Config,
    posts: Vec<Content>,
    category: String,
}

pub struct Generator {
    config: Config,
    paths: Paths,
    templates: TemplateEngine,
}

impl Generator {
    pub fn new(config: Config, paths: Paths) -> Result<Self> {
        let templates = TemplateEngine::new(&paths.templates_dir)?;

        Ok(Self {
            config,
            paths,
            templates,
        })
    }

    pub fn build(&mut self) -> Result<()> {
        println!("Building site...");

        // Clean output directory
        if self.paths.output_dir.exists() {
            fs::remove_dir_all(&self.paths.output_dir)?;
        }
        fs::create_dir_all(&self.paths.output_dir)?;

        // Load all content
        let mut all_content = self.load_all_content()?;

        // Generate URLs for all content
        self.generate_urls(&mut all_content)?;

        // Load data files
        let data = self.load_data_files()?;

        // Render pages
        self.render_pages(&all_content, &data)?;

        // Render posts
        self.render_posts(&all_content)?;

        // Copy static files
        self.copy_static_files()?;

        println!("Site built successfully!");
        Ok(())
    }

    fn load_all_content(&self) -> Result<Vec<Content>> {
        let mut content = Vec::new();

        for entry in WalkDir::new(&self.paths.content_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            // Load markdown and HTML files as content
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md" || ext == "html") {
                match Content::from_file(path, &self.paths.content_dir) {
                    Ok(c) => content.push(c),
                    Err(e) => eprintln!("Warning: Failed to parse {}: {}", path.display(), e),
                }
            }
        }

        Ok(content)
    }

    fn generate_urls(&self, content: &mut [Content]) -> Result<()> {
        for item in content.iter_mut() {
            let url = if let (Some(category), Some(date)) = (&item.category, &item.date) {
                // Use permalink pattern for posts
                let pattern = self.config.get_permalink_pattern(category);
                let mut url = pattern
                    .replace(":year", &date.format("%Y").to_string())
                    .replace(":month", &date.format("%m").to_string())
                    .replace(":day", &date.format("%d").to_string())
                    .replace(":slug", &item.slug);

                // Add .html extension if not present
                if !url.ends_with(".html") {
                    url.push_str(".html");
                }
                url
            } else {
                // Regular page
                let mut url = format!("/{}", item.slug);
                if !url.ends_with(".html") {
                    url.push_str(".html");
                }
                url
            };

            item.url = url;
        }

        Ok(())
    }

    fn render_pages(&self, all_content: &[Content], data: &HashMap<String, serde_json::Value>) -> Result<()> {
        let pages: Vec<_> = all_content.iter()
            .filter(|c| c.category.is_none())
            .collect();

        // Group posts by category for template access
        let mut posts_by_category: HashMap<String, Vec<Content>> = HashMap::new();
        for post in all_content.iter().filter(|c| c.category.is_some()) {
            posts_by_category
                .entry(post.category.clone().unwrap())
                .or_default()
                .push(post.clone());
        }

        // Sort posts by date (newest first)
        for posts in posts_by_category.values_mut() {
            posts.sort_by(|a, b| b.date.cmp(&a.date));
        }

        for page in pages {
            let context = PageContext {
                site: self.config.clone(),
                page: page.clone(),
                posts_by_category: posts_by_category.clone(),
                data: data.clone(),
            };

            let layout = page.frontmatter.layout.as_deref().unwrap_or("page.html");
            let html = self.templates.render(layout, &context)?;

            // Use the URL to determine output path (already has .html extension)
            let output_file = page.url.trim_start_matches('/');
            let output_path = self.paths.output_dir.join(output_file);

            fs::write(&output_path, html)
                .with_context(|| format!("Failed to write page: {:?}", output_path))?;

            println!("Generated: {}", output_path.display());
        }

        Ok(())
    }

    fn render_posts(&self, all_content: &[Content]) -> Result<()> {
        let mut posts_by_category: HashMap<String, Vec<&Content>> = HashMap::new();

        for post in all_content.iter().filter(|c| c.category.is_some()) {
            posts_by_category
                .entry(post.category.clone().unwrap())
                .or_default()
                .push(post);
        }

        // Sort posts by date (newest first)
        for posts in posts_by_category.values_mut() {
            posts.sort_by(|a, b| b.date.cmp(&a.date));
        }

        // Render individual posts
        for (category, posts) in &posts_by_category {
            for post in posts {
                let context = PostContext {
                    site: self.config.clone(),
                    page: (*post).clone(),
                };

                let layout = post.frontmatter.layout.as_deref().unwrap_or("post.html");
                let html = self.templates.render(layout, &context)?;

                // Create output directory structure
                let output_file = post.url.trim_start_matches('/');
                let output_path = self.paths.output_dir.join(output_file);

                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(&output_path, html)
                    .with_context(|| format!("Failed to write post: {:?}", output_path))?;

                println!("Generated: {}", output_path.display());
            }

            // Generate category index
            let context = ListContext {
                site: self.config.clone(),
                posts: posts.iter().map(|p| (*p).clone()).collect(),
                category: category.clone(),
            };

            let html = self.templates.render("list.html", &context)?;
            let output_path = self.paths.output_dir.join(category).join("index.html");

            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&output_path, html)?;
            println!("Generated category index: {}", output_path.display());
        }

        Ok(())
    }

    fn copy_static_files(&self) -> Result<()> {
        if !self.paths.static_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(&self.paths.static_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let relative = path.strip_prefix(&self.paths.static_dir)?;
                let output_path = self.paths.output_dir.join(relative);

                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::copy(path, &output_path)?;
                println!("Copied: {}", relative.display());
            }
        }

        Ok(())
    }

    fn load_data_files(&self) -> Result<HashMap<String, serde_json::Value>> {
        let mut data = HashMap::new();

        if !self.paths.data_dir.exists() {
            return Ok(data);
        }

        for entry in WalkDir::new(&self.paths.data_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let file_stem = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    match ext {
                        "json" => {
                            // Only try to read text-based formats
                            if let Ok(contents) = fs::read_to_string(path) {
                                if let Ok(json) = serde_json::from_str(&contents) {
                                    data.insert(file_stem.to_string(), json);
                                }
                            }
                        }
                        "csv" => {
                            if let Ok(contents) = fs::read_to_string(path) {
                                // Parse CSV into JSON array
                                let mut reader = csv::Reader::from_reader(contents.as_bytes());
                                let mut records = Vec::new();

                                if let Ok(headers) = reader.headers() {
                                    let headers: Vec<String> = headers.iter().map(|h| h.to_string()).collect();

                                    for result in reader.records() {
                                        if let Ok(record) = result {
                                            let mut obj = serde_json::Map::new();
                                            for (i, field) in record.iter().enumerate() {
                                                if let Some(header) = headers.get(i) {
                                                    obj.insert(header.clone(), serde_json::Value::String(field.to_string()));
                                                }
                                            }
                                            records.push(serde_json::Value::Object(obj));
                                        }
                                    }
                                }

                                data.insert(file_stem.to_string(), serde_json::Value::Array(records));
                            }
                        }
                        _ => {
                            // Skip binary files and unknown formats
                        }
                    }
                }
            }
        }

        Ok(data)
    }

    pub fn reload_templates(&mut self) -> Result<()> {
        self.templates.reload()
    }
}
