# ajrudzitis.github.io

Personal website for Aleks Rudzitis, built with a custom static site generator written in Rust.

**Live site:** [aleksrudzitis.com](https://aleksrudzitis.com)

## Overview

This repository contains both the static site generator and the content for ajrudzitis.com. The site was migrated from a Ruby-based generator to this custom Rust implementation for better performance and maintainability.

## Features

- **Custom Rust-based static site generator** - Fast, reliable builds with minimal dependencies
- **Markdown and HTML support** - Write content in either format with YAML frontmatter
- **Category-based organization** - Automatically organize posts by category (letters, articles, etc.)
- **Flexible permalinks** - Configurable URL patterns per category
- **Tera templating** - Powerful template engine with inheritance and partials
- **Development server** - Hot-reload for rapid iteration during development
- **Data files** - Load JSON/CSV data files for use in templates

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Cargo

### Build the site

```bash
cd static-site-generator
cargo build --release
cargo run -- build
```

Output will be generated in the `public/` directory.

### Development server

Start a local server with hot-reload:

```bash
cd static-site-generator
cargo run -- serve --port 8000
```

Visit [http://localhost:8000](http://localhost:8000) to view the site. Changes to files in `site/` will trigger automatic rebuilds.

## Project Structure

```
.
├── static-site-generator/     # Rust static site generator
│   └── src/
│       ├── main.rs            # CLI entry point
│       ├── config.rs          # Configuration loading
│       ├── content.rs         # Content parsing (markdown, frontmatter)
│       ├── generator.rs       # Core build logic
│       ├── template.rs        # Template engine wrapper
│       ├── server.rs          # Development server
│       └── watcher.rs         # File watching
├── site/                      # Site content and configuration
│   ├── config.toml            # Site configuration
│   ├── content/               # Pages and posts
│   │   ├── pages/             # Top-level pages
│   │   └── posts/             # Posts organized by category
│   ├── templates/             # Tera templates
│   ├── static/                # Static assets (CSS, images, etc.)
│   └── data/                  # Data files (JSON, CSV)
└── public/                    # Generated output (not in version control)
```

## Writing Content

### Pages

Create files in `site/content/pages/`:

```markdown
---
title: "About"
layout: "page.html"
---

# About Me

Your content here...
```

Pages are rendered at the root level: `/about.html`

### Posts

Create files in `site/content/posts/{category}/`:

```
site/content/posts/letters/2024-07-18-hello-world.md
```

Filename format: `YYYY-MM-DD-slug.md`

Posts support the same frontmatter as pages. The category and date are extracted from the file path.

### Frontmatter

All content files support YAML frontmatter:

```yaml
---
title: "Post Title"
layout: "custom.html"  # Optional, defaults to page.html or post.html
date: "2024-07-18"     # Optional for posts (extracted from filename)
---
```

## Configuration

Edit `site/config.toml` to customize:

```toml
title = "Your Site Title"
description = "Your site description"
base_url = "https://yoursite.com"
author = "Your Name"

[social]
twitter = "handle"
github = "username"
linkedin = "username"

[permalinks]
letters = "/letters/:year-:month-:day"
articles = "/articles/:year-:month-:day-:slug"
```

Permalink variables: `:year`, `:month`, `:day`, `:slug`

## CLI Usage

### Build command

```bash
cargo run -- build [OPTIONS]

Options:
  --root <PATH>         Path to site directory (default: site/)
  --output-dir <PATH>   Output directory (default: public/)
```

### Serve command

```bash
cargo run -- serve [OPTIONS]

Options:
  --root <PATH>         Path to site directory (default: site/)
  --output-dir <PATH>   Output directory (default: public/)
  --port <PORT>         Port to serve on (default: 8000)
```

## Deployment

The site is designed to be deployed as static files. Simply build the site and deploy the contents of `public/` to your hosting provider.

## License

This is a personal website. The static site generator code is provided as-is for reference, but not licensed for reuse.

## Contributing

This is a personal website repository. While issues and suggestions are welcome, pull requests will generally not be accepted.
