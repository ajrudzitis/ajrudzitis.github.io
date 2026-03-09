# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal website (ajrudzitis.com) built with a custom static site generator written in Rust. The project migrated from a Ruby-based generator to this Rust implementation.

## Build Commands

### Build the site
```bash
cd static-site-generator
cargo build --release
cargo run -- build
```

### Development server with hot-reload
```bash
cd static-site-generator
cargo run -- serve --port 8000
```

### Building from repository root
The binary defaults to looking for `site/` directory:
```bash
cd static-site-generator
cargo run -- build
# or
cargo run -- serve
```

### Specifying custom paths
```bash
cargo run -- build --root /path/to/site --output-dir /path/to/output
cargo run -- serve --root /path/to/site --output-dir /path/to/output --port 8000
```

## Architecture

### Directory Structure

```
/
├── static-site-generator/     # Rust SSG implementation
│   └── src/
│       ├── main.rs            # CLI entry point, build & serve commands
│       ├── lib.rs             # Module exports
│       ├── config.rs          # Config & Paths structs, loads config.toml
│       ├── content.rs         # Content parsing, frontmatter, markdown->HTML
│       ├── generator.rs       # Core build logic, URL generation, rendering
│       ├── template.rs        # Tera template engine wrapper
│       ├── server.rs          # Axum dev server
│       └── watcher.rs         # File watching for hot-reload
└── site/                      # Site content
    ├── config.toml            # Site configuration
    ├── content/
    │   ├── pages/             # Top-level pages (no category)
    │   └── posts/
    │       └── {category}/    # Posts organized by category (e.g., letters/)
    ├── templates/             # Tera templates (*.html)
    ├── static/                # Static assets (copied as-is to output)
    └── data/                  # Data files (JSON, CSV)
```

### Core Build Flow

1. **Config Loading** (config.rs): Loads `site/config.toml` with site metadata, social links, and permalink patterns
2. **Content Loading** (generator.rs:load_all_content): Recursively walks `content/` directory, parsing `.md` and `.html` files
3. **Content Parsing** (content.rs): Extracts YAML frontmatter, converts markdown to HTML, determines category/date/slug from file path
4. **URL Generation** (generator.rs:generate_urls): Applies permalink patterns to posts, generates URLs for pages
5. **Rendering** (generator.rs:render_pages, render_posts): Uses Tera templates to render HTML, passes context with site config and content
6. **Static Files** (generator.rs:copy_static_files): Copies everything from `static/` to output directory
7. **Data Files** (generator.rs:load_data_files): Loads JSON/CSV from `data/` directory, available in templates as `data.*`

### Content Organization

- **Pages**: Files in `content/pages/` become top-level pages at `/{slug}.html`
- **Posts**: Files in `content/posts/{category}/` become categorized posts
  - Filename format: `YYYY-MM-DD-slug.md` (date extraction is automatic)
  - URL pattern: Defined in `config.toml` permalinks (e.g., `/letters/:year-:month-:day.html`)
  - Category index: Generated at `/{category}/index.html`

Current categories:
- **letters/**: Personal newsletter posts (synced with Buttondown via `buttondown-cli`)
- **articles/**: Standalone articles (not synced with Buttondown)

### Frontmatter

Files can include YAML frontmatter:
```yaml
---
title: "Post Title"
layout: "custom.html"  # Optional, defaults to page.html or post.html
date: "2024-07-18"     # Optional, can also be extracted from filename
---
```

HTML files without frontmatter extract title from `<title>` tags (see content.rs:extract_html_title).

### Template System

Uses Tera templating engine. Templates receive context:
- **Pages**: `PageContext` with `site`, `page`, `posts_by_category`, `data`
- **Posts**: `PostContext` with `site`, `page`
- **Category lists**: `ListContext` with `site`, `posts`, `category`

Templates:
- `base.html`: Base layout (typically extended by other templates)
- `page.html`: Default page layout
- `post.html`: Default post layout
- `list.html`: Category index layout
- `home.html`, `bookshelf.html`: Custom page layouts
- `partials/`: Reusable template fragments

### Development Server

The serve command (main.rs:serve):
1. Performs initial build
2. Starts file watcher on `site/` directory
3. Launches Axum dev server on specified port (default 8000)
4. Rebuilds on file changes with live reload

## Working with the Codebase

### Adding Features

Most changes fall into these categories:

1. **Content processing**: Modify `content.rs` (frontmatter parsing, markdown options)
2. **Build logic**: Modify `generator.rs` (URL patterns, rendering context, output structure)
3. **Template features**: Modify `template.rs` (custom filters, functions)
4. **CLI options**: Modify `main.rs` (new commands, arguments)

### Common Patterns

- **Error handling**: Uses `anyhow::Result` throughout, context added with `.with_context()`
- **Path handling**: `Paths` struct in config.rs centralizes directory locations
- **Content iteration**: Posts sorted by date (newest first) before rendering
- **Template lookup**: Layout specified in frontmatter or defaults based on content type

### Configuration

Site config in `site/config.toml`:
```toml
title = "Site Title"
description = "Site description"
base_url = "https://example.com"
author = "Author Name"

[social]
twitter = "handle"
github = "username"
linkedin = "username"

[permalinks]
category_name = "/path/:year-:month-:day"
```

Permalink variables: `:year`, `:month`, `:day`, `:slug`
