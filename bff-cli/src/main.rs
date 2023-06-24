use std::path::PathBuf;

use clap::*;
use crc32::{Crc32Algorithm, Crc32Format, Crc32Mode};
use crc64::{Crc64Algorithm, Crc64Format, Crc64Mode};
use lz::LzEndian;

mod crc32;
mod crc64;
mod extract;
mod info;
mod lz;

#[derive(Subcommand)]
enum Commands {
    Extract {
        bigfile: PathBuf,
        directory: PathBuf,
    },
    Info {
        bigfile: PathBuf,
    },
    Crc32 {
        string: Option<String>,
        #[arg(
            short,
            long,
            default_value_t = 0,
            help = "Starting value for the CRC-32 calculation"
        )]
        starting: u32,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc32Algorithm::Asobo)]
        algorithm: Crc32Algorithm,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc32Mode::Lines)]
        mode: Crc32Mode,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc32Format::Unsigned)]
        format: Crc32Format,
    },
    Crc64 {
        string: Option<String>,
        #[arg(
            short,
            long,
            default_value_t = 0,
            help = "Starting value for the CRC-32 calculation"
        )]
        starting: u64,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc64Algorithm::Asobo)]
        algorithm: Crc64Algorithm,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc64Mode::Lines)]
        mode: Crc64Mode,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc64Format::Unsigned)]
        format: Crc64Format,
    },
    Lz {
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: LzEndian,
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
        Commands::Crc32 {
            string,
            starting,
            algorithm,
            mode,
            format,
        } => crc32::crc32(string, starting, algorithm, mode, format),
        Commands::Crc64 {
            string,
            starting,
            algorithm,
            mode,
            format,
        } => crc64::crc64(string, starting, algorithm, mode, format),
        Commands::Lz { endian } => lz::lz(endian),
    }
}
