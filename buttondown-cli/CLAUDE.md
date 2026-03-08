# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A CLI tool for syncing local newsletter "letters" (markdown/HTML files) with Buttondown email service. Part of the ajrudzitis.github.io personal website project.

## Build Commands

```bash
cargo build --release
cargo run -- <command>
```

No tests are currently configured.

## CLI Usage

```bash
# List remote emails
cargo run -- list
cargo run -- list --status draft --format json

# Download an email as a local letter file
cargo run -- download <email-id>

# View sync status (compare local vs remote)
cargo run -- sync --verbose

# Match existing remote emails to local letters
cargo run -- backfill --dry-run --verbose

# Push a new letter as draft
cargo run -- push path/to/letter.md

# Update an already-linked letter
cargo run -- update path/to/letter.md
```

Global flags: `--dry-run`, `--verbose`, `--letters-dir <PATH>`, `--api-key-file <PATH>`

## Architecture

### Module Structure

| Module | Purpose |
|--------|---------|
| `main.rs` | CLI entry point (clap), command handlers, output formatting |
| `api.rs` | Buttondown REST API client with pagination |
| `config.rs` | Config loading, API key resolution (file → env var) |
| `models.rs` | Data structures: `LocalLetter`, `ButtondownEmail`, `SyncState` |
| `letter.rs` | Letter file parsing, frontmatter extraction/writing |
| `sync.rs` | Compares local letters with remote emails |
| `backfill.rs` | Auto-matching algorithm (slug → title → date) |

### Key Data Flow

1. **LocalLetter**: Parsed from `site/content/posts/letters/` files (default path)
   - Filename format: `YYYY-MM-DD-slug.md` (date/slug extracted automatically)
   - YAML frontmatter contains `title`, optional `buttondown_id`

2. **ButtondownEmail**: Fetched from Buttondown API (`GET /emails`)

3. **Sync tracking**: `buttondown_id` in frontmatter links local letter to remote email
   - `download` fetches email and creates local file with `buttondown_id`
   - `push` creates draft and writes `buttondown_id` to frontmatter
   - `backfill` matches existing emails by slug/title/date and writes `buttondown_id`
   - `update` requires `buttondown_id` to already exist

### Safety Constraints

- `create_email()` always sets `status: "draft"` (never auto-sends)
- `update_email()` never modifies status field
- All write operations support `--dry-run`

### API Key Loading Order

1. `--api-key-file` argument
2. `.buttondown-api-key` file in current directory
3. `BUTTONDOWN_API_KEY` environment variable
