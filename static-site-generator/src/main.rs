use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Duration;

use static_site_generator::{Config, Generator};
use static_site_generator::config::Paths;
use static_site_generator::server::DevServer;
use static_site_generator::watcher::Watcher;

#[derive(Parser)]
#[command(name = "ssg")]
#[command(about = "A static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Root directory (the site directory containing config.toml)
    #[arg(short, long)]
    root: Option<PathBuf>,

    /// Path to the output directory
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the site
    Build,

    /// Serve the site with hot-reload
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value_t = 8000)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine site directory (root is the site directory)
    let site_dir = if let Some(root) = cli.root {
        std::fs::canonicalize(&root)?
    } else {
        // Default: look for site/ in current directory, or use current directory if config.toml exists
        let current = std::env::current_dir()?;
        let site_candidate = current.join("site");

        if site_candidate.join("config.toml").exists() {
            site_candidate
        } else if current.join("config.toml").exists() {
            current
        } else {
            site_candidate // fallback to site/ even if it doesn't exist yet
        }
    };

    // Determine output directory
    let output_dir = if let Some(out) = cli.output_dir {
        std::fs::canonicalize(&out).unwrap_or_else(|_| out)
    } else {
        // Default: public/ next to the site directory
        site_dir.parent()
            .unwrap_or(&site_dir)
            .join("public")
    };

    match cli.command {
        Commands::Build => build(&site_dir, &output_dir)?,
        Commands::Serve { port } => serve(&site_dir, &output_dir, port).await?,
    }

    Ok(())
}

fn build(site_dir: &PathBuf, output_dir: &PathBuf) -> Result<()> {
    let config = Config::load(site_dir)?;
    let paths = Paths::new(site_dir.clone(), output_dir.clone());
    let mut generator = Generator::new(config, paths)?;

    generator.build()?;

    Ok(())
}

async fn serve(site_dir: &PathBuf, output_dir: &PathBuf, port: u16) -> Result<()> {
    // Initial build
    println!("Performing initial build...");
    build(site_dir, output_dir)?;

    // Set up file watcher
    let mut watcher = Watcher::new()?;
    watcher.watch(site_dir)?;

    println!("\nWatching for changes in {:?}...", site_dir);

    // Start dev server in background
    let server = DevServer::new(output_dir.clone(), port);
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.run().await {
            eprintln!("Server error: {}", e);
        }
    });

    // Watch for file changes
    loop {
        if let Some(events) = watcher.check_for_changes(Duration::from_millis(500)) {
            println!("\nDetected {} change(s), rebuilding...", events.len());

            match build(site_dir, output_dir) {
                Ok(_) => println!("Rebuild complete!"),
                Err(e) => eprintln!("Build failed: {}", e),
            }
        }

        // Check if server is still running
        if server_handle.is_finished() {
            break;
        }
    }

    Ok(())
}
