use anyhow::Result;
use colored::Colorize;

use crate::models::{ButtondownEmail, LocalLetter, MatchResult};
use crate::sync::find_matches;

/// Run backfill operation to match existing emails to local letters
pub fn run_backfill(
    letters: &mut [LocalLetter],
    emails: &[ButtondownEmail],
    dry_run: bool,
    verbose: bool,
) -> Result<BackfillResult> {
    // Filter to only untracked letters
    let untracked: Vec<_> = letters
        .iter()
        .filter(|l| l.buttondown_id.is_none())
        .cloned()
        .collect();

    if untracked.is_empty() {
        println!("All local letters already have buttondown_id");
        return Ok(BackfillResult::default());
    }

    let (matches, unmatched_letters, unmatched_emails) = find_matches(&untracked, emails);

    // Print results
    if !matches.is_empty() {
        println!(
            "\n{} {}:",
            if dry_run { "Would match" } else { "Matched" },
            matches.len()
        );
        for m in &matches {
            let filename = m.local.path.file_name().unwrap_or_default().to_string_lossy();
            println!(
                "  {} {} -> \"{}\" ({})",
                if dry_run { "[DRY-RUN]".yellow() } else { "[MATCHED]".green() },
                filename,
                m.remote.subject,
                format!("by {}", m.match_type).dimmed()
            );
            if verbose {
                println!("    ID: {}", m.remote.id);
            }
        }
    }

    if !unmatched_letters.is_empty() {
        println!("\n{} Unmatched local letters:", "!".yellow());
        for letter in &unmatched_letters {
            let filename = letter.path.file_name().unwrap_or_default().to_string_lossy();
            println!("  {} {}", "-".red(), filename);
        }
    }

    if !unmatched_emails.is_empty() {
        println!("\n{} Unmatched remote emails:", "!".yellow());
        for email in &unmatched_emails {
            println!(
                "  {} \"{}\" ({})",
                "-".blue(),
                email.subject,
                email.id.dimmed()
            );
        }
    }

    // Actually write the buttondown_id if not dry-run
    if !dry_run {
        for m in &matches {
            // Find the letter in the mutable slice and update it
            if let Some(letter) = letters.iter_mut().find(|l| l.path == m.local.path) {
                letter.write_buttondown_id(&m.remote.id)?;
                if verbose {
                    println!(
                        "  {} Updated frontmatter for {}",
                        "[WROTE]".green(),
                        letter.path.display()
                    );
                }
            }
        }
    }

    Ok(BackfillResult {
        matches,
        unmatched_letters,
        unmatched_emails,
    })
}

#[derive(Debug, Default)]
pub struct BackfillResult {
    pub matches: Vec<MatchResult>,
    pub unmatched_letters: Vec<LocalLetter>,
    pub unmatched_emails: Vec<ButtondownEmail>,
}
