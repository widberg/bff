use std::path::PathBuf;

use clap::*;
use crc32::{Crc32Algorithm, CrcFormat, CrcMode};
use crc64::Crc64Algorithm;
use error::BffCliResult;
use lz::LzEndian;

mod crc32;
mod crc64;
mod error;
mod extract;
mod info;
mod lz;
mod unlz;

#[derive(Subcommand)]
enum Commands {
    #[clap(alias = "x")]
    Extract {
        bigfile: PathBuf,
        directory: PathBuf,
        #[arg(long)]
        in_names: Vec<PathBuf>,
        #[arg(long)]
        out_names: Option<PathBuf>,
    },
    #[clap(alias = "t")]
    Info {
        bigfile: PathBuf,
        #[arg(long)]
        in_names: Vec<PathBuf>,
    },
    Crc32 {
        string: Option<String>,
        #[arg(
            short,
            long,
            default_value_t = 0,
            help = "Starting value for the CRC-32 calculation"
        )]
        starting: i32,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc32Algorithm::Asobo)]
        algorithm: Crc32Algorithm,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = CrcMode::Lines)]
        mode: CrcMode,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = CrcFormat::Signed)]
        format: CrcFormat,
    },
    Crc64 {
        string: Option<String>,
        #[arg(
            short,
            long,
            default_value_t = 0,
            help = "Starting value for the CRC-32 calculation"
        )]
        starting: i64,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = Crc64Algorithm::Asobo)]
        algorithm: Crc64Algorithm,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = CrcMode::Lines)]
        mode: CrcMode,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = CrcFormat::Signed)]
        format: CrcFormat,
    },
    Unlz {
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: lz::LzEndian,
    },
    Lz {
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: lz::LzEndian,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> BffCliResult<()> {
    let cli = Args::parse();

    match &cli.command {
        Commands::Extract {
            bigfile,
            directory,
            in_names,
            out_names,
        } => extract::extract(bigfile, directory, in_names, out_names),
        Commands::Info { bigfile, in_names } => info::info(bigfile, in_names),
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
        Commands::Unlz { endian } => unlz::unlz(endian),
        Commands::Lz { endian } => lz::lz(endian),
    }
}
