//use atolldb::db;
use atolldb::crypto;
use atolldb::utils;

use clap::{Parser, Subcommand};
use anyhow::Result;

/// Command-line interface for atolldb
#[derive(Parser)]
#[command(name = "atolldb", version = env!("CARGO_PKG_VERSION"), about = "atolldb CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// All supported CLI commands
#[derive(Subcommand)]
enum Commands {
    /// Create a new atolldb file with header
    Create {
        #[arg(short = 'p', long, default_value = "atolldb.adb")]
        file: String,
    },

    /// Insert a JSON document into a collection
    Insert {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        data: String,

        #[arg(short = 'p', long, default_value = "atolldb.adb")]
        file: String,
    },

    /// Query documents from a collection
    Find {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        filter: String,

        #[arg(short = 'p', long, default_value = "atolldb.adb")]
        file: String,
    },

    /// Update documents in a collection
    Update {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        filter: String,

        #[arg(short, long)]
        update: String,

        #[arg(short = 'p', long, default_value = "atolldb.adb")]
        file: String,
    },

    /// Delete documents from a collection
    Delete {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        filter: String,

        #[arg(short = 'p', long, default_value = "atolldb.adb")]
        file: String,
    },

    /// Define or update a schema for a collection
    Schema {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        schema: String,

        #[arg(short = 'p', long, default_value = "atolldb.adb")]
        file: String,
    },

    /// Show version and banner
    #[command(name = "version", alias = "-v")]
    Version,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    utils::set_debug(true); // Enable debug logging

    match cli.command {
        Commands::Create { file } => {
            db::create(&file)?;
            utils::print_ferris();
            println!("✅ Created new atolldb file: {}", file);
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
        Commands::Update { collection, filter, update, file } => {
            let filter_json: serde_json::Value = serde_json::from_str(&filter)?;
            let update_json: serde_json::Value = serde_json::from_str(&update)?;
            let count = db::update(&file, &collection, &filter_json, &update_json)?;
            println!("🔁 Updated {} document(s) in '{}'", count, collection);
        }
        Commands::Delete { collection, filter, file } => {
            let filter_json: serde_json::Value = serde_json::from_str(&filter)?;
            let count = db::delete(&file, &collection, &filter_json)?;
            println!("🗑️ Deleted {} document(s) from '{}'", count, collection);
        }
        Commands::Schema { collection, schema, file } => {
            let schema_json: serde_json::Value = serde_json::from_str(&schema)?;
            db::set_schema(&file, &collection, &schema_json)?;
            println!("📐 Schema defined for collection '{}'", collection);
        }
        Commands::Version => {
            utils::print_ferris();
        }
    }

    Ok(())
}
