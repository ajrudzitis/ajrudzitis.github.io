# aleksrudzitis.com TODO List

This document tracks planned work for the website and related infrastructure.

---

## Content Organization & Cleanup

### ~~Move Article to Letters~~ Integrate Articles as Category
- [x] Move `2019-06-30-thats-not-what-happened.html` from `site/static/articles/` to `site/content/posts/articles/`
  - Moved to content system with proper frontmatter
  - Articles category added to top navigation
  - Articles use slug-based URLs (e.g., `/articles/2019-06-30-thats-not-what-happened.html`)
  - Articles are not synced with Buttondown (unlike letters)

### Remove Legacy Links
- [x] Remove Tinyletter links from 7 old letter archives (2020-2021 HTML files)
  - Files affected:
    - `2020-10-03-koan.html`
    - `2020-11-25-the-sum-of-our-parts.html`
    - `2020-12-19-2020-christmas-letter.html`
    - `2021-03-28-spring-2021.html`
    - `2021-05-29-the-fading-scent-of-lilacs.html`
    - `2021-06-13-this-old-house.html`
    - `2021-08-17-among-the-trees.html`
  - **Done**: All tinyletter links removed during proofreading project

---

## Buttondown Email Integration

### CLI Sync Tool
- [x] Design and implement Buttondown CLI sync tool architecture
  - Manual command-based workflow (not automated)
  - Should support selective sync of letters
  - **Done**: `buttondown-cli` tool created with list, download, sync, backfill, push, update commands

- [x] Implement Buttondown API authentication and article push functionality
  - API documentation: https://api.buttondown.email/
  - Support creating/updating emails via API
  - **Done**: Full API integration with push (create drafts) and update functionality

- [x] Add metadata tracking for which letters are published to Buttondown
  - Uses `buttondown_id` frontmatter field to link local letters to remote emails
  - **Done**: backfill command auto-matches existing emails, sync tracks state

---

## Proofreading Project

Review all letters and articles for grammatical errors, misspellings, and misplaced words.

- [x] All 61 letters and 1 article proofread
  - **Done**: See `PROOFREADING_CHECKLIST.md` for full details
  - Corrections synced to Buttondown for all letters with `buttondown_id`
  - Added `revised_on` frontmatter to files with corrections

---

## Terminal/TUI Version

Build a terminal interface for browsing letters, accessible via both web (easter egg) and SSH.

### Architecture Overview

The terminal app will be a single Rust application compiled to two targets:
1. **WASM** - Runs in browser via the terminal easter egg (Ctrl+Shift+X)
2. **Native** - Runs on server, exposed via SSH

This allows the same TUI codebase to power both experiences.

### Browser Terminal (WASM)
- [x] Create terminal easter egg infrastructure
  - **Done**: `site/static/js/terminal-egg.js` provides xterm.js terminal overlay
  - Triggered by Ctrl+Shift+X, full-screen blue terminal aesthetic
  - Currently shows random quotes; ready to connect to WASM backend

- [ ] Compile TUI app to WASM
  - Use `wasm-bindgen` or `wasm-pack` to compile Rust TUI to WebAssembly
  - Create JS glue code to bridge xterm.js input/output to WASM module
  - The `processCommand()` function in `terminal-egg.js` is the integration point

- [ ] Integration points in `terminal-egg.js`:
  - Replace `processCommand(input)` with calls to WASM module
  - WASM module should expose: `init()`, `process_input(string)`, `get_output() -> string`
  - Handle async rendering (WASM -> terminal output)

### Core TUI Application
- [ ] Design TUI/terminal app architecture
  - Browse all letters with arrow key navigation
  - Read letters in terminal with scrolling
  - Search functionality
  - Technology: Ratatui (Rust) for TUI rendering

- [ ] Implement TUI letter browser with navigation and reading interface
  - Abstract I/O layer to support both native terminal and xterm.js
  - Clean reading experience with word wrap and scrolling

- [ ] Build content loading for TUI
  - Load letter content (can be embedded at compile time or fetched)
  - Parse markdown to terminal-friendly format

### SSH Server (Native)
- [ ] Implement SSH server wrapper for TUI app
  - Use `russh` or similar for SSH protocol
  - Each connection spawns TUI instance
  - Should support multiple concurrent connections

- [ ] Test TUI app locally and gather feedback
  - Ensure good UX for terminal navigation
  - Test on various terminal emulators

---

## Website Styling & Design

Iterate on website design with theme switching between retro and minimalist aesthetics. Inspired by post-modern art museums.

### Theme System
- [x] Design website style iteration: explore retro/nostalgic + minimalist themes
  - **Done**: Created 4 themes covering the aesthetic spectrum
  - CSS architecture: `base.css` (structural) + `themes/*.css` (visual)

- [x] Implement theme switcher mechanism (retro vs minimalist modes)
  - **Done**: Keyboard shortcut `Ctrl+Shift+T` / `Cmd+Shift+T` cycles through themes
  - Persistent user preference via localStorage
  - Flash-free loading via inline script in `<head>`
  - Toast notification shows current theme name

### Visual Themes
- [x] Develop retro/nostalgic aesthetic option with post-modern art influences
  - **Done**: Created 3 retro-inspired themes:
    - `terminal` - CRT/hacker aesthetic (green on dark, monospace, glow effects, scanlines)
    - `brutalist` - 90s web (bold colors, Times New Roman, harsh borders, GeoCities energy)
    - `postmodern` - MONA-inspired (irreverent typography, subtle rotations, grain texture)

- [x] Develop minimalist refined aesthetic option with excellent typography
  - **Done**: `minimalist` theme with system fonts, clean borders, subtle colors

### Interactive Elements
- [x] Add playful easter eggs and interactive JavaScript toys to website
  - **Done**: Implemented in `site/static/js/easter-eggs.js`:
    - Konami code (↑↑↓↓←→←→BA) triggers floating quote bubble
    - 7 footer clicks reveals random quote
    - Console greeting for developers
    - Idle meditation prompt after 2 minutes
  - 48 whimsical quotes in `site/static/quotes/quotes.txt`
  - All easter egg UI styled to match current theme

- [x] Add terminal easter egg (Ctrl+Shift+X)
  - **Done**: Implemented in `site/static/js/terminal-egg.js`:
    - Full-screen blue terminal overlay using xterm.js
    - Press Enter for random quotes, type `help` for commands
    - ESC or `exit` to close
  - Foundation for future WASM-based terminal app (see Terminal/TUI Version section)

### Future Enhancements
- [ ] Add visible theme toggle button (optional, keyboard shortcut is primary)
- [ ] Consider additional themes (e.g., `sepia`, `high-contrast`, `seasonal`)
- [ ] Add more quotes to `quotes.txt`
- [ ] Consider additional easter eggs (e.g., secret page, cursor trails)

---

## AWS Deployment Infrastructure

Deploy TUI app to AWS with automated deployment pipeline.

### NixOS Configuration
- [ ] Create NixOS configuration for TUI app deployment
  - Package TUI app for NixOS
  - Define system configuration

- [ ] Build NixOS image for AWS deployment
  - Create AMI from NixOS configuration
  - Optimize for small instance size

### AWS Setup
- [ ] Set up AWS infrastructure (EC2 instance, security groups, networking)
  - Provision EC2 instance (small size)
  - Configure security groups for SSH/telnet access
  - Set up networking and domain routing
  - Decision needed: AWS region preference

### CI/CD
- [ ] Create GitHub Actions workflow for TUI app deployment to AWS
  - Build NixOS image
  - Deploy to EC2
  - Auto-update on push to main/master

- [ ] Test end-to-end deployment pipeline and verify TUI app accessibility
  - Verify public accessibility
  - Test deployment automation
  - Monitor for issues

---

## Open Questions

1. ~~**Buttondown metadata format**: Should we use `buttondown_url`, `buttondown_id`, or `published_to_buttondown: true` in frontmatter?~~ **Resolved**: Using `buttondown_id` in frontmatter
2. **TUI network protocol**: SSH (secure, auth required) or telnet (simpler, fully public)?
3. **AWS region/instance**: Preferred region and instance size for TUI deployment?
4. ~~**Theme switching UI**: Toggle button, keyboard shortcut, or both?~~ **Resolved**: Keyboard shortcut only (`Ctrl+Shift+T`) for clean interface; themes are discoverable by power users

---

*Last updated: 2026-03-14*
