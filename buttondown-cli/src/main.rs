use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use comfy_table::{Cell, Table};
use std::path::PathBuf;

use buttondown_cli::api::ButtondownClient;
use buttondown_cli::backfill::run_backfill;
use buttondown_cli::config::Config;
use buttondown_cli::letter::load_letters;
use buttondown_cli::models::SyncState;
use buttondown_cli::sync::compare_letters_and_emails;

#[derive(Parser)]
#[command(name = "buttondown-cli")]
#[command(about = "Manage Buttondown email synchronization", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Show what would happen without making changes
    #[arg(short, long, global = true)]
    dry_run: bool,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Path to letters directory
    #[arg(short, long, global = true)]
    letters_dir: Option<PathBuf>,

    /// Path to API key file [default: .buttondown-api-key]
    #[arg(short = 'k', long, global = true)]
    api_key_file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// List emails from Buttondown
    List {
        /// Filter by status (draft, scheduled, sent, imported)
        #[arg(short, long)]
        status: Option<String>,

        /// Output format (table, json)
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Get details of a specific email by ID
    Get {
        /// Email ID
        id: String,
    },

    /// Push a local letter to Buttondown as draft
    Push {
        /// Path to the letter file
        file: PathBuf,
    },

    /// Update existing email from local letter
    Update {
        /// Path to the letter file (must have buttondown_id in frontmatter)
        file: PathBuf,
    },

    /// Compare local letters with Buttondown emails
    Sync {
        /// Download all remote-only emails as local letters
        #[arg(long)]
        download: bool,
    },

    /// Match existing Buttondown emails to local letters
    Backfill,

    /// Download email from Buttondown and save as local letter
    Download {
        /// Email ID to download
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load(cli.api_key_file, cli.letters_dir, cli.dry_run, cli.verbose)?;

    match cli.command {
        Commands::List { status, format } => {
            list_emails(&config, status.as_deref(), &format).await?;
        }
        Commands::Get { id } => {
            get_email(&config, &id).await?;
        }
        Commands::Push { file } => {
            push_letter(&config, &file).await?;
        }
        Commands::Update { file } => {
            update_letter(&config, &file).await?;
        }
        Commands::Sync { download } => {
            sync_status(&config, download).await?;
        }
        Commands::Backfill => {
            backfill(&config).await?;
        }
        Commands::Download { id } => {
            download_email(&config, &id).await?;
        }
    }

    Ok(())
}

async fn list_emails(config: &Config, status: Option<&str>, format: &str) -> Result<()> {
    let client = ButtondownClient::new(config);
    let emails = client.list_emails(status).await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&emails)?);
        return Ok(());
    }

    if emails.is_empty() {
        println!("No emails found");
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec!["ID", "Subject", "Status", "Date"]);

    for email in &emails {
        let date = email
            .publish_date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .or_else(|| email.creation_date.map(|d| d.format("%Y-%m-%d").to_string()))
            .unwrap_or_else(|| "-".to_string());

        let status_cell = match email.status.as_str() {
            "sent" => Cell::new(&email.status).fg(comfy_table::Color::Green),
            "draft" => Cell::new(&email.status).fg(comfy_table::Color::Yellow),
            "scheduled" => Cell::new(&email.status).fg(comfy_table::Color::Blue),
            _ => Cell::new(&email.status),
        };

        table.add_row(vec![
            Cell::new(&email.id),
            Cell::new(&email.subject),
            status_cell,
            Cell::new(date),
        ]);
    }

    println!("{table}");
    println!("\nTotal: {} emails", emails.len());

    Ok(())
}

async fn get_email(config: &Config, id: &str) -> Result<()> {
    let client = ButtondownClient::new(config);
    let email = client.get_email(id).await?;

    println!("{}: {}", "ID".bold(), email.id);
    println!("{}: {}", "Subject".bold(), email.subject);
    println!("{}: {}", "Status".bold(), email.status);

    if let Some(slug) = &email.slug {
        println!("{}: {}", "Slug".bold(), slug);
    }

    if let Some(date) = email.publish_date {
        println!("{}: {}", "Published".bold(), date.format("%Y-%m-%d %H:%M"));
    }

    if let Some(date) = email.creation_date {
        println!("{}: {}", "Created".bold(), date.format("%Y-%m-%d %H:%M"));
    }

    println!("\n{}", "Body:".bold());
    println!("{}", email.body);

    Ok(())
}

async fn push_letter(config: &Config, file: &PathBuf) -> Result<()> {
    use buttondown_cli::models::LocalLetter;

    let mut letter = LocalLetter::from_file(file)
        .with_context(|| format!("Failed to read letter: {:?}", file))?;

    if letter.buttondown_id.is_some() {
        println!(
            "{} This letter already has a buttondown_id. Use 'update' command instead.",
            "Warning:".yellow()
        );
        return Ok(());
    }

    if config.dry_run {
        println!("{}", "[DRY-RUN] Would push letter:".yellow());
        println!("  Title: {}", letter.title);
        println!("  File: {:?}", file);
        println!("  Status: draft (always)");
        return Ok(());
    }

    let client = ButtondownClient::new(config);
    let email = client.create_email(&letter.title, &letter.body).await?;

    println!("{} Created draft email", "[SUCCESS]".green());
    println!("  ID: {}", email.id);
    println!("  Subject: {}", email.subject);

    // Update frontmatter with buttondown_id
    letter.write_buttondown_id(&email.id)?;
    println!("  {} Added buttondown_id to frontmatter", "[UPDATED]".green());

    Ok(())
}

async fn update_letter(config: &Config, file: &PathBuf) -> Result<()> {
    use buttondown_cli::models::LocalLetter;

    let letter = LocalLetter::from_file(file)
        .with_context(|| format!("Failed to read letter: {:?}", file))?;

    let buttondown_id = letter.buttondown_id.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "Letter does not have a buttondown_id. Use 'push' command first, or 'backfill' to match existing emails."
        )
    })?;

    if config.dry_run {
        println!("{}", "[DRY-RUN] Would update email:".yellow());
        println!("  ID: {}", buttondown_id);
        println!("  Title: {}", letter.title);
        println!("  File: {:?}", file);
        return Ok(());
    }

    let client = ButtondownClient::new(config);
    let email = client
        .update_email(buttondown_id, &letter.title, &letter.body)
        .await?;

    println!("{} Updated email", "[SUCCESS]".green());
    println!("  ID: {}", email.id);
    println!("  Subject: {}", email.subject);

    Ok(())
}

async fn sync_status(config: &Config, download: bool) -> Result<()> {
    let letters = load_letters(&config.letters_dir)?;
    let client = ButtondownClient::new(config);
    let emails = client.list_emails(None).await?;

    let states = compare_letters_and_emails(&letters, &emails);

    let mut matched = 0;
    let mut local_only = 0;
    let mut remote_only_emails = Vec::new();

    for state in &states {
        match state {
            SyncState::Matched { local, remote } => {
                matched += 1;
                if config.verbose {
                    let filename = local.path.file_name().unwrap_or_default().to_string_lossy();
                    println!(
                        "{} {} <-> \"{}\"",
                        "[MATCHED]".green(),
                        filename,
                        remote.subject
                    );
                }
            }
            SyncState::LocalOnly(letter) => {
                local_only += 1;
                let filename = letter.path.file_name().unwrap_or_default().to_string_lossy();
                println!("{} {} (no buttondown_id)", "[LOCAL]".yellow(), filename);
            }
            SyncState::RemoteOnly(email) => {
                remote_only_emails.push(email.clone());
                println!(
                    "{} \"{}\" ({})",
                    "[REMOTE]".blue(),
                    email.subject,
                    &email.id
                );
            }
        }
    }

    println!("\n{}", "Summary:".bold());
    println!("  Matched: {}", matched);
    println!("  Local only: {}", local_only);
    println!("  Remote only: {}", remote_only_emails.len());

    if local_only > 0 {
        println!(
            "\n{}: Run 'backfill' to match local letters with remote emails, or 'push' to create new drafts.",
            "Tip".cyan()
        );
    }

    // Download remote-only emails if requested
    if download && !remote_only_emails.is_empty() {
        if config.dry_run {
            println!("\n{}", "[DRY-RUN] Would download:".yellow());
            for email in &remote_only_emails {
                match get_email_file_path(config, email) {
                    Ok((_, path)) => {
                        println!("  \"{}\" -> {}", email.subject, path.display());
                    }
                    Err(e) => {
                        println!("  \"{}\" - {}", email.subject, e);
                    }
                }
            }
        } else {
            println!("\n{}", "Downloading remote-only emails:".bold());
            let mut downloaded = 0;
            let mut skipped = 0;

            for email in &remote_only_emails {
                match save_email_as_letter(config, email) {
                    Ok(Some(path)) => {
                        downloaded += 1;
                        println!(
                            "  {} \"{}\" -> {}",
                            "[DOWNLOADED]".green(),
                            email.subject,
                            path.display()
                        );
                    }
                    Ok(None) => {
                        skipped += 1;
                        if config.verbose {
                            println!(
                                "  {} \"{}\" (already exists)",
                                "[SKIPPED]".yellow(),
                                email.subject
                            );
                        }
                    }
                    Err(e) => {
                        println!(
                            "  {} \"{}\" - {}",
                            "[ERROR]".red(),
                            email.subject,
                            e
                        );
                    }
                }
            }

            println!("\n  Downloaded: {}, Skipped: {}", downloaded, skipped);
        }
    }

    Ok(())
}

async fn backfill(config: &Config) -> Result<()> {
    let mut letters = load_letters(&config.letters_dir)?;
    let client = ButtondownClient::new(config);
    let emails = client.list_emails(None).await?;

    println!(
        "Found {} local letters and {} remote emails",
        letters.len(),
        emails.len()
    );

    let result = run_backfill(&mut letters, &emails, config.dry_run, config.verbose)?;

    println!("\n{}", "Summary:".bold());
    println!("  Already matched: {}", result.already_matched);
    println!("  Newly matched: {}", result.new_matches.len());
    println!("  Unmatched local: {}", result.unmatched_letters.len());
    println!("  Unmatched remote: {}", result.unmatched_emails.len());

    if config.dry_run && !result.new_matches.is_empty() {
        println!(
            "\n{}: Run without --dry-run to write buttondown_id to frontmatter.",
            "Tip".cyan()
        );
    }

    Ok(())
}

async fn download_email(config: &Config, id: &str) -> Result<()> {
    let client = ButtondownClient::new(config);
    let email = client.get_email(id).await?;

    if config.dry_run {
        let (_, file_path) = get_email_file_path(config, &email)?;
        println!("{}", "[DRY-RUN] Would download email:".yellow());
        println!("  ID: {}", email.id);
        println!("  Subject: {}", email.subject);
        println!("  File: {}", file_path.display());
        return Ok(());
    }

    match save_email_as_letter(config, &email)? {
        Some(path) => {
            println!("{} Downloaded email", "[SUCCESS]".green());
            println!("  ID: {}", email.id);
            println!("  Subject: {}", email.subject);
            println!("  File: {}", path.display());
        }
        None => {
            let (_, file_path) = get_email_file_path(config, &email)?;
            println!(
                "{} File already exists: {}",
                "Warning:".yellow(),
                file_path.display()
            );
        }
    }

    Ok(())
}

/// Get the filename and path for an email
fn get_email_file_path(
    config: &Config,
    email: &buttondown_cli::models::ButtondownEmail,
) -> Result<(String, std::path::PathBuf)> {
    let date = email
        .publish_date
        .or(email.creation_date)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .ok_or_else(|| anyhow::anyhow!("Email has no publish_date or creation_date"))?;

    let slug = email
        .slug
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| slugify(&email.subject));

    let filename = format!("{}-{}.md", date, slug);
    let file_path = config.letters_dir.join(&filename);

    Ok((filename, file_path))
}

/// Save an email as a local letter file. Returns Some(path) if saved, None if already exists.
fn save_email_as_letter(
    config: &Config,
    email: &buttondown_cli::models::ButtondownEmail,
) -> Result<Option<std::path::PathBuf>> {
    use buttondown_cli::models::Frontmatter;
    use std::collections::HashMap;

    let (_, file_path) = get_email_file_path(config, email)?;

    if file_path.exists() {
        return Ok(None);
    }

    // Build frontmatter using proper YAML serialization
    let frontmatter = Frontmatter {
        title: email.subject.clone(),
        layout: None,
        date: None,
        buttondown_id: Some(email.id.clone()),
        extra: HashMap::new(),
    };
    let yaml = serde_yaml::to_string(&frontmatter)
        .with_context(|| "Failed to serialize frontmatter")?;
    let content = format!("---\n{}---\n\n{}", yaml, email.body);

    std::fs::write(&file_path, &content)
        .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

    Ok(Some(file_path))
}

/// Convert a title to a URL-friendly slug
fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
