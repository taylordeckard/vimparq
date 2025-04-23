mod edit;
mod view;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vimparq", about = "CLI Parquet Viewer/Editor")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Edit a Parquet file
    Edit {
        /// Path to the Parquet file
        parquet_path: PathBuf,
        /// Path to the JSONL file containing edits
        json_path: PathBuf,
    },
    /// View a Parquet file and print it to stdout
    View {
        /// Path to the Parquet file
        path: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Edit { parquet_path, json_path } => edit::edit_parquet(&parquet_path, &json_path)?,
        Commands::View { path } => view::view_parquet(&path)?,
    }

    Ok(())
}
