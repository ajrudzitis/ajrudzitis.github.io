# Migration from Jekyll to Custom Rust SSG

This document outlines the migration of aleksrudzitis.com from Jekyll to a custom Rust-based static site generator.

## What Changed

### Directory Structure

**Old (Jekyll):**
```
├── _config.yml
├── _posts/
├── _layouts/
├── _includes/
└── assets/
```

**New:**
```
├── static-site-generator/    # Rust generator
├── site/                     # Site-specific content
│   ├── config.toml
│   ├── content/
│   │   ├── pages/
│   │   └── posts/
│   ├── templates/
│   ├── static/
│   └── data/
└── public/                   # Generated output
```

### Technology Stack

- **Build Tool:** Jekyll → Custom Rust SSG
- **Templating:** Liquid → Tera (similar syntax)
- **Content:** Markdown + YAML frontmatter (unchanged)
- **Deployment:** GitHub Pages (unchanged)

## URLs Preserved

All URLs remain identical to maintain compatibility:
- Posts: `/letters/YYYY-MM-DD.html`
- Posts: `/articles/YYYY-MM-DD.html`
- Pages: `/page-name.html`

## Development Workflow

### Building the Site

```bash
cd static-site-generator
cargo run --release -- build
```

### Local Development

```bash
cargo run --release -- serve
```

Visit `http://localhost:8000` to preview the site. Changes are watched and the site rebuilds automatically.

## Deployment

GitHub Actions automatically builds and deploys the site when pushing to the `master` branch.

See `.github/workflows/deploy.yml` for the deployment configuration.

## Features Added

1. **Hot-reload development server** - Automatically rebuilds and serves changes
2. **Type-safe configuration** - Rust's type system prevents configuration errors
3. **Faster builds** - Rust's performance advantage over Ruby
4. **Simplified dependencies** - No need for Ruby, Bundler, or Jekyll gems

## Next Steps

### Remaining Tasks

1. **Enhance dynamic content** - Implement data file integration for bookshelf
2. **Theme customization** - Update CSS to match Poolsuite-inspired aesthetic
3. **Template improvements** - Convert remaining Jekyll Liquid templates
4. **Add CLI commands** - `ssg new post` to create new posts easily

### Future Enhancements

- Syntax highlighting for code blocks
- Image optimization
- RSS feed generation
- Sitemap generation
- Search functionality

## Notes

- The old Jekyll files remain in the repository for reference
- You can delete Jekyll-specific files once fully migrated
- The bookshelf currently shows static content; dynamic generation needs implementation
