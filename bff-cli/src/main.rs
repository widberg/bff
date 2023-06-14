use std::path::PathBuf;

use clap::*;

mod extract;
mod info;

#[derive(Subcommand)]
enum Commands {
    Extract {
        bigfile: PathBuf,
        directory: PathBuf,
    },
    Info {
        bigfile: PathBuf,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();

    match &cli.command {
        Commands::Extract { bigfile, directory } => extract::extract(bigfile, directory),
        Commands::Info { bigfile } => info::info(bigfile),
    }
}
