use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Email from Buttondown API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtondownEmail {
    pub id: String,
    pub subject: String,
    #[serde(default)]
    pub body: String,
    pub status: String,
    #[serde(default)]
    pub slug: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub publish_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub secondary_id: Option<i64>,
}

/// Response from listing emails
#[derive(Debug, Deserialize)]
pub struct EmailListResponse {
    pub count: i64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<ButtondownEmail>,
}

/// Local letter parsed from filesystem
#[derive(Debug, Clone)]
pub struct LocalLetter {
    pub path: std::path::PathBuf,
    pub title: String,
    pub body: String,
    pub html: String,
    pub date: Option<NaiveDate>,
    pub slug: String,
    pub buttondown_id: Option<String>,
    pub frontmatter: Frontmatter,
}

/// Frontmatter from a letter file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttondown_id: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// Sync state between local and remote
#[derive(Debug, Clone)]
pub enum SyncState {
    /// Local letter has no buttondown_id, not matched yet
    LocalOnly(LocalLetter),
    /// Remote email has no matching local letter
    RemoteOnly(ButtondownEmail),
    /// Matched pair (local has buttondown_id)
    Matched {
        local: LocalLetter,
        remote: ButtondownEmail,
    },
}

/// Result of a match attempt during backfill
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub local: LocalLetter,
    pub remote: ButtondownEmail,
    pub match_type: MatchType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Slug,
    Title,
    Date,
}

impl std::fmt::Display for MatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchType::Slug => write!(f, "slug"),
            MatchType::Title => write!(f, "title"),
            MatchType::Date => write!(f, "date"),
        }
    }
}

/// Request to create a new email (always as draft)
#[derive(Debug, Serialize)]
pub struct CreateEmailRequest {
    pub subject: String,
    pub body: String,
    pub status: String, // Always "draft"
}

/// Request to update an existing email (excludes status for safety)
#[derive(Debug, Serialize)]
pub struct UpdateEmailRequest {
    pub subject: String,
    pub body: String,
}
