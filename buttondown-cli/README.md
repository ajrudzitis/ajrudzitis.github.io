# buttondown-cli

A CLI tool for syncing local newsletter "letters" with [Buttondown](https://buttondown.email).

## Setup

1. Build the CLI:
   ```bash
   cargo build --release
   ```

2. Create a `.buttondown-api-key` file with your Buttondown API key:
   ```bash
   echo "your-api-key-here" > .buttondown-api-key
   ```

   Alternatively, set the `BUTTONDOWN_API_KEY` environment variable.

## Usage

```bash
# List remote emails
buttondown-cli list
buttondown-cli list --status draft
buttondown-cli list --format json

# View a specific email
buttondown-cli get <email-id>

# Download an email as a local letter file
buttondown-cli download <email-id>
buttondown-cli download --dry-run <email-id>

# Compare local letters with remote emails
buttondown-cli sync
buttondown-cli sync --verbose

# Auto-match existing remote emails to local letters
buttondown-cli backfill --dry-run
buttondown-cli backfill

# Push a new local letter as a draft
buttondown-cli push path/to/letter.md

# Update an already-linked letter
buttondown-cli update path/to/letter.md
```

### Global Options

| Option | Description |
|--------|-------------|
| `--dry-run`, `-d` | Preview changes without making them |
| `--verbose`, `-v` | Show detailed output |
| `--letters-dir`, `-l` | Custom letters directory (default: `site/content/posts/letters`) |
| `--api-key-file`, `-k` | Custom API key file (default: `.buttondown-api-key`) |

## Letter Format

Letters are markdown files with YAML frontmatter:

```markdown
---
title: "Letter Title"
buttondown_id: "em_abc123..."
---

Letter content here...
```

Filename format: `YYYY-MM-DD-slug.md` (e.g., `2024-03-08-hello-world.md`)

## Commands

### `list`
Lists emails from Buttondown. Filter by status (`draft`, `sent`, `scheduled`, `imported`) or output as JSON.

### `get`
Shows details of a specific email by ID.

### `download`
Downloads an email from Buttondown and saves it as a local letter file with the correct naming convention and frontmatter.

### `sync`
Compares local letters with remote emails and shows:
- **Matched**: Local letter linked to remote email via `buttondown_id`
- **Local only**: Letter without `buttondown_id` (not yet pushed or matched)
- **Remote only**: Email with no matching local letter

### `backfill`
Automatically matches existing Buttondown emails to local letters using three strategies (in order):
1. Slug matching
2. Title matching
3. Date matching

Writes `buttondown_id` to matched letters' frontmatter.

### `push`
Creates a new draft email in Buttondown from a local letter. Writes the `buttondown_id` back to the letter's frontmatter.

### `update`
Updates an existing Buttondown email from a local letter. Requires the letter to have a `buttondown_id` in its frontmatter.

## Safety

- `push` always creates emails as **drafts** to prevent accidental sends
- `update` never modifies email status
- All write operations support `--dry-run`
