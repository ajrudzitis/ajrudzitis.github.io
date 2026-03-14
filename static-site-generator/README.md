# Static Site Generator

A custom static site generator written in Rust, designed for aleksrudzitis.com.

## Features

- Markdown content with YAML frontmatter
- Tera templating (Jinja2-like syntax)
- Automatic URL generation matching Jekyll patterns
- Hot-reload development server
- Fast builds with Rust
- Multi-theme system with keyboard switching
- Easter eggs and hidden interactions

## Usage

### Build the site

```bash
# From the project root
cargo run --release -- build

# Or from anywhere by specifying the root
cargo run --release -- --root /path/to/site build
```

### Development server with hot-reload

```bash
# From the project root
cargo run --release -- serve

# Or from anywhere
cargo run --release -- --root /path/to/site serve
```

The dev server will run on `http://localhost:8000` by default. Use `-p` to specify a different port:

```bash
cargo run --release -- serve -p 3000
```

## Options

- `-r, --root <PATH>` - Root directory for the site (default: `.`)
- `-s, --site-dir <PATH>` - Path to site directory relative to root (default: `site`)
- `-o, --output-dir <PATH>` - Path to output directory relative to root (default: `public`)

### Examples

```bash
# Build from current directory (uses ./site and ./public)
ssg build

# Build from specific root directory
ssg --root ~/my-website build

# Use custom site and output directories
ssg --site-dir content --output-dir dist build

# Combine root with custom directories
ssg --root ~/my-website --output-dir _build build
```

## Architecture

- **config.rs** - Site configuration loading
- **content.rs** - Markdown and frontmatter parsing
- **template.rs** - Tera template rendering
- **generator.rs** - Main site generation logic
- **server.rs** - Development server
- **watcher.rs** - File watching for hot-reload

## Content Structure

Place content in `site/content/`:
- `pages/` - Static pages (e.g., index.md, about.md)
- `posts/<category>/` - Blog posts organized by category

Posts should follow the naming pattern: `YYYY-MM-DD-slug.md`

## Templates

Templates are located in `site/templates/` and use Tera syntax.

Available context variables:
- `site` - Site configuration
- `page` - Current page/post content and metadata
- `posts` - List of posts (in list templates)

## Theming

The site supports four visual themes, switchable via `Ctrl+Shift+T` (or `Cmd+Shift+T` on Mac):

| Theme | Description |
|-------|-------------|
| `minimalist` | Clean, system fonts, white background |
| `terminal` | CRT/hacker aesthetic, green monospace on dark |
| `brutalist` | 90s web, bold colors, Times New Roman |
| `postmodern` | MONA-inspired, playful typography (default) |

Theme preference is saved in localStorage. Configure the default in `config.toml`:

```toml
[theme]
default = "postmodern"
```

Theme CSS files are in `site/static/css/themes/`. The base structural styles (shared across all themes) are in `site/static/css/base.css`.

## Easter Eggs

Hidden features activated by:
- **Konami Code** (↑↑↓↓←→←→BA): Floating quote bubble
- **Footer clicks** (7x): Random quote reveal
- **Idle** (2 min): Gentle mindfulness prompt

Quotes are loaded from `site/static/quotes/quotes.txt`.

## License

MIT
