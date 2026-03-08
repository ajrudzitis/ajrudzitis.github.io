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
    Sync,

    /// Match existing Buttondown emails to local letters
    Backfill,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { status, format } => {
            let config = Config::load(cli.api_key_file, cli.letters_dir, cli.dry_run, cli.verbose)?;
            list_emails(&config, status.as_deref(), &format).await?;
        }
        Commands::Get { id } => {
            let config = Config::load(cli.api_key_file, cli.letters_dir, cli.dry_run, cli.verbose)?;
            get_email(&config, &id).await?;
        }
        Commands::Push { file } => {
            let config = Config::load(cli.api_key_file, cli.letters_dir, cli.dry_run, cli.verbose)?;
            push_letter(&config, &file).await?;
        }
        Commands::Update { file } => {
            let config = Config::load(cli.api_key_file, cli.letters_dir, cli.dry_run, cli.verbose)?;
            update_letter(&config, &file).await?;
        }
        Commands::Sync => {
            let config = Config::load(cli.api_key_file, cli.letters_dir, cli.dry_run, cli.verbose)?;
            sync_status(&config).await?;
        }
        Commands::Backfill => {
            let config = Config::load(cli.api_key_file, cli.letters_dir, cli.dry_run, cli.verbose)?;
            backfill(&config).await?;
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
            Cell::new(&email.id[..8.min(email.id.len())]),
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

async fn sync_status(config: &Config) -> Result<()> {
    let letters = load_letters(&config.letters_dir)?;
    let client = ButtondownClient::new(config);
    let emails = client.list_emails(None).await?;

    let states = compare_letters_and_emails(&letters, &emails);

    let mut matched = 0;
    let mut local_only = 0;
    let mut remote_only = 0;

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
                remote_only += 1;
                println!(
                    "{} \"{}\" ({})",
                    "[REMOTE]".blue(),
                    email.subject,
                    &email.id[..8.min(email.id.len())]
                );
            }
        }
    }

    println!("\n{}", "Summary:".bold());
    println!("  Matched: {}", matched);
    println!("  Local only: {}", local_only);
    println!("  Remote only: {}", remote_only);

    if local_only > 0 {
        println!(
            "\n{}: Run 'backfill' to match local letters with remote emails, or 'push' to create new drafts.",
            "Tip".cyan()
        );
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
    println!("  Matched: {}", result.matches.len());
    println!("  Unmatched local: {}", result.unmatched_letters.len());
    println!("  Unmatched remote: {}", result.unmatched_emails.len());

    if config.dry_run && !result.matches.is_empty() {
        println!(
            "\n{}: Run without --dry-run to write buttondown_id to frontmatter.",
            "Tip".cyan()
        );
    }

    Ok(())
}
