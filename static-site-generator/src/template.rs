use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;
use tera::Tera;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(templates_dir: &Path) -> Result<Self> {
        let pattern = templates_dir
            .join("**/*.html")
            .to_string_lossy()
            .to_string();

        let mut tera = Tera::new(&pattern)
            .with_context(|| format!("Failed to load templates from {}", pattern))?;

        // Add custom filters if needed
        tera.autoescape_on(vec!["html"]);

        Ok(Self { tera })
    }

    pub fn render<T: Serialize>(&self, template_name: &str, context: &T) -> Result<String> {
        let ctx = tera::Context::from_serialize(context)
            .with_context(|| "Failed to serialize context")?;

        self.tera
            .render(template_name, &ctx)
            .with_context(|| format!("Failed to render template: {}", template_name))
    }

    pub fn reload(&mut self) -> Result<()> {
        self.tera.full_reload()
            .with_context(|| "Failed to reload templates")
    }
}
