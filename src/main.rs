mod db;
mod crypto;
mod utils;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "jasdb", version = "0.1.0", about = "JasDB CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Insert a JSON document
    Insert {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        data: String,
    },
    /// Query documents
    Find {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        filter: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Insert { collection, data } => {
            let doc: serde_json::Value = serde_json::from_str(&data)?;
            db::insert(&collection, &doc)?;
            println!("✅ Document inserted into '{}'", collection);
        }
        Commands::Find { collection, filter } => {
            let query: serde_json::Value = serde_json::from_str(&filter)?;
            let results = db::query(&collection, &query)?;
            println!("📦 Results:\n{}", serde_json::to_string_pretty(&results)?);
        }
    }

    Ok(())
}
