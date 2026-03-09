# aleksrudzitis.com TODO List

This document tracks planned work for the website and related infrastructure.

---

## Content Organization & Cleanup

### Move Article to Letters
- [ ] Move `2019-06-30-thats-not-what-happened.html` from `site/static/articles/` to `site/content/posts/letters/`
  - Currently in static directory as standalone HTML
  - Should be integrated into letters collection
  - File: `/site/static/articles/2019-06-30-thats-not-what-happened.html`

### Remove Legacy Links
- [ ] Remove Tinyletter links from 7 old letter archives (2020-2021 HTML files)
  - Files affected:
    - `2020-10-03-koan.html`
    - `2020-11-25-the-sum-of-our-parts.html`
    - `2020-12-19-2020-christmas-letter.html`
    - `2021-03-28-spring-2021.html`
    - `2021-05-29-the-fading-scent-of-lilacs.html`
    - `2021-06-13-this-old-house.html`
    - `2021-08-17-among-the-trees.html`

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

Review all 37 letters for grammatical errors, misspellings, and misplaced words. **Each batch requires approval before proceeding.**

- [ ] Proofreading batch 1: Review first 3-5 letters for grammatical errors (requires approval)
- [ ] Proofreading batch 2: Review next 3-5 letters for grammatical errors (requires approval)
- [ ] Proofreading batch 3: Review next 3-5 letters for grammatical errors (requires approval)
- [ ] Proofreading batch 4: Review next 3-5 letters for grammatical errors (requires approval)
- [ ] Proofreading batch 5: Review next 3-5 letters for grammatical errors (requires approval)
- [ ] Proofreading batch 6: Review next 3-5 letters for grammatical errors (requires approval)
- [ ] Proofreading batch 7: Review remaining letters for grammatical errors (requires approval)

### Sync Corrections
- [ ] Sync proofread letter revisions to Buttondown for previously published letters
  - Use `buttondown-cli update` to push revisions to published versions
  - Only sync letters that have `buttondown_id` in frontmatter

---

## Terminal/TUI Version

Build a network-accessible terminal interface for browsing letters.

### Core TUI Application
- [ ] Design TUI/terminal app architecture (network-accessible, letter browsing)
  - Browse all letters with arrow key navigation
  - Read letters in terminal
  - Network accessible (SSH or telnet)

- [ ] Implement TUI letter browser with navigation and reading interface
  - Technology options: Ratatui (Rust), Bubble Tea (Go), or similar
  - Clean reading experience in terminal

- [ ] Implement TUI network server (SSH or telnet access)
  - Decision needed: SSH (secure, requires auth) vs telnet (simple, public)
  - Should support multiple concurrent connections

- [ ] Test TUI app locally and gather feedback
  - Ensure good UX for terminal navigation
  - Test on various terminal emulators

---

## Website Styling & Design

Iterate on website design with theme switching between retro and minimalist aesthetics. Inspired by post-modern art museums.

### Theme System
- [ ] Design website style iteration: explore retro/nostalgic + minimalist themes
  - Research post-modern art museum aesthetics
  - Create design mockups for both themes

- [ ] Implement theme switcher mechanism (retro vs minimalist modes)
  - Persistent user preference (localStorage)
  - Smooth transitions between themes

### Visual Themes
- [ ] Develop retro/nostalgic aesthetic option with post-modern art influences
  - Options: 90s web, terminal-inspired, brutalist, vintage computing
  - Post-modern art museum influence

- [ ] Develop minimalist refined aesthetic option with excellent typography
  - Clean, sophisticated design
  - Excellent spacing and readability
  - Subtle details

### Interactive Elements
- [ ] Add playful easter eggs and interactive JavaScript toys to website
  - Hidden surprises
  - Interactive experiments
  - Fun discoveries for visitors

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
4. **Theme switching UI**: Toggle button, keyboard shortcut, or both?

---

*Last updated: 2026-03-08*
