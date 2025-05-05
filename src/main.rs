mod db;
mod crypto;
mod utils;

use clap::{Parser, Subcommand};
use anyhow::Result;

/// Command-line interface for JasDB
#[derive(Parser)]
#[command(name = "jasdb", version = "0.1.0", about = "JasDB CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// All supported CLI commands
#[derive(Subcommand)]
enum Commands {
    /// Create a new JasDB file with header
    Create {
        #[arg(short = 'p', long, default_value = "jasdb.jasdb")]
        file: String,
    },

    /// Insert a JSON document into a collection
    Insert {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        data: String,

        #[arg(short = 'p', long, default_value = "jasdb.jasdb")]
        file: String,
    },

    /// Query documents from a collection
    Find {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        filter: String,

        #[arg(short = 'p', long, default_value = "jasdb.jasdb")]
        file: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { file } => {
            db::create(&file)?;
            println!("✅ Created new JasDB file: {}", file);
        }
        Commands::Insert { collection, data, file } => {
            let doc: serde_json::Value = serde_json::from_str(&data)?;
            db::insert(&file, &collection, &doc)?;
            println!("✅ Document inserted into '{}'", collection);
        }
        Commands::Find { collection, filter, file } => {
            let query: serde_json::Value = serde_json::from_str(&filter)?;
            let results = db::query(&file, &collection, &query)?;
            println!("📦 Results:\n{}", serde_json::to_string_pretty(&results)?);
        }
    }

    Ok(())
}
