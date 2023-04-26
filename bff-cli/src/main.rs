use std::path::PathBuf;

use clap::*;

mod extract;

#[derive(Subcommand)]
enum Commands {
    Extract {
        #[arg(short, long, required = true)]
        bigfile: PathBuf,
        #[arg(short, long, required = true)]
        directory: PathBuf,
    }
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
        Commands::Extract { bigfile, directory } => {
            extract::extract(bigfile, directory)
        }
    }
}
