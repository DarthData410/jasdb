mod db;
mod crypto;
mod utils;

use clap::{Parser, Subcommand};
use anyhow::Result;

/// Command-line interface for JasDB
#[derive(Parser)]
#[command(name = "jasdb", version = "0.1.0", about = "JasDB CLI")]
struct Cli {
    /// Optional path to the database file
    #[arg(short, long, default_value = "jasdb.jasdb")]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

/// All supported CLI commands
#[derive(Subcommand)]
enum Commands {
    /// Insert a JSON document into a collection
    Insert {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        data: String,
    },
    /// Query documents from a collection
    Find {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        filter: String,
    }
}

fn main() -> Result<()> {
    // Parse CLI input into the Cli struct
    let cli = Cli::parse();

    match cli.command {
        Commands::Insert { collection, data } => {
            // Deserialize the input data into a JSON value
            let doc: serde_json::Value = serde_json::from_str(&data)?;

            // NEW: Pass the user-supplied file path to the insert function
            db::insert(&cli.file, &collection, &doc)?;

            println!("✅ Document inserted into '{}'", collection);
        }
        Commands::Find { collection, filter } => {
            // Deserialize the filter string into a JSON value
            let query: serde_json::Value = serde_json::from_str(&filter)?;

            // NEW: Pass the user-supplied file path to the query function
            let results = db::query(&cli.file, &collection, &query)?;

            println!("📦 Results:\n{}", serde_json::to_string_pretty(&results)?);
        }
    }

    Ok(())
}
