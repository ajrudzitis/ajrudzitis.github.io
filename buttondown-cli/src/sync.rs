use std::collections::HashSet;

use crate::letter::normalize_for_comparison;
use crate::models::{ButtondownEmail, LocalLetter, MatchResult, MatchType, SyncState};

/// Compare local letters with remote emails and return sync state
pub fn compare_letters_and_emails(
    letters: &[LocalLetter],
    emails: &[ButtondownEmail],
) -> Vec<SyncState> {
    let mut states = Vec::new();

    // Track which emails have been matched
    let mut matched_email_ids: HashSet<String> = HashSet::new();

    // First, handle letters that already have buttondown_id
    for letter in letters {
        if let Some(ref id) = letter.buttondown_id {
            if let Some(email) = emails.iter().find(|e| &e.id == id) {
                states.push(SyncState::Matched {
                    local: letter.clone(),
                    remote: email.clone(),
                });
                matched_email_ids.insert(id.clone());
            } else {
                // Has ID but email not found (deleted?)
                states.push(SyncState::LocalOnly(letter.clone()));
            }
        } else {
            states.push(SyncState::LocalOnly(letter.clone()));
        }
    }

    // Add unmatched emails
    for email in emails {
        if !matched_email_ids.contains(&email.id) {
            states.push(SyncState::RemoteOnly(email.clone()));
        }
    }

    states
}

/// Try to match unmatched local letters with unmatched remote emails
pub fn find_matches(
    letters: &[LocalLetter],
    emails: &[ButtondownEmail],
) -> (Vec<MatchResult>, Vec<LocalLetter>, Vec<ButtondownEmail>) {
    let mut matches = Vec::new();
    let mut unmatched_letters: Vec<LocalLetter> = Vec::new();
    let mut matched_email_ids: HashSet<String> = HashSet::new();

    // Only consider letters without buttondown_id
    let untracked_letters: Vec<_> = letters
        .iter()
        .filter(|l| l.buttondown_id.is_none())
        .collect();

    for letter in &untracked_letters {
        let mut found_match = false;

        // Try to match by slug
        if let Some(email) = emails
            .iter()
            .find(|e| !matched_email_ids.contains(&e.id) && matches_by_slug(letter, e))
        {
            matches.push(MatchResult {
                local: (*letter).clone(),
                remote: email.clone(),
                match_type: MatchType::Slug,
            });
            matched_email_ids.insert(email.id.clone());
            found_match = true;
        }

        // Try to match by title
        if !found_match
            && let Some(email) = emails
                .iter()
                .find(|e| !matched_email_ids.contains(&e.id) && matches_by_title(letter, e))
        {
            matches.push(MatchResult {
                local: (*letter).clone(),
                remote: email.clone(),
                match_type: MatchType::Title,
            });
            matched_email_ids.insert(email.id.clone());
            found_match = true;
        }

        // Try to match by date
        if !found_match
            && let Some(email) = emails
                .iter()
                .find(|e| !matched_email_ids.contains(&e.id) && matches_by_date(letter, e))
        {
            matches.push(MatchResult {
                local: (*letter).clone(),
                remote: email.clone(),
                match_type: MatchType::Date,
            });
            matched_email_ids.insert(email.id.clone());
            found_match = true;
        }

        if !found_match {
            unmatched_letters.push((*letter).clone());
        }
    }

    // Collect unmatched emails
    let unmatched_emails: Vec<_> = emails
        .iter()
        .filter(|e| !matched_email_ids.contains(&e.id))
        .cloned()
        .collect();

    (matches, unmatched_letters, unmatched_emails)
}

fn matches_by_slug(letter: &LocalLetter, email: &ButtondownEmail) -> bool {
    if let Some(ref email_slug) = email.slug {
        // Normalize both slugs for comparison
        let letter_slug = letter.slug.to_lowercase().replace('_', "-");
        let email_slug = email_slug.to_lowercase().replace('_', "-");
        letter_slug == email_slug
    } else {
        false
    }
}

fn matches_by_title(letter: &LocalLetter, email: &ButtondownEmail) -> bool {
    let letter_title = normalize_for_comparison(&letter.title);
    let email_title = normalize_for_comparison(&email.subject);
    !letter_title.is_empty() && letter_title == email_title
}

fn matches_by_date(letter: &LocalLetter, email: &ButtondownEmail) -> bool {
    if let (Some(letter_date), Some(email_date)) = (&letter.date, &email.publish_date) {
        letter_date == &email_date.date_naive()
    } else {
        false
    }
}
