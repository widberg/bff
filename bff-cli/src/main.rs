use std::path::PathBuf;

use clap::*;
use crc::{CrcAlgorithm, CrcFormat, CrcMode};
use error::BffCliResult;
use lz::LzEndian;
use reverse_crc32::DEFAULT_CHARACTER_SET;

use crate::lz::LzAlgorithm;

mod crc;
mod csc;
mod error;
mod extract;
mod info;
mod lz;
mod reverse_crc32;
mod round_trip;
mod psc;

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
    #[clap(alias = "rt")]
    RoundTrip {
        bigfile: PathBuf,
    },
    #[clap(alias = "crc32", alias = "crc64")]
    Crc {
        string: Option<String>,
        #[arg(short, long, help = "Starting value for the CRC calculation")]
        starting: Option<i64>,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = CrcAlgorithm::Asobo)]
        algorithm: CrcAlgorithm,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = CrcMode::Lines)]
        mode: CrcMode,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = CrcFormat::Signed)]
        format: CrcFormat,
    },
    #[clap(alias = "rcrc32")]
    ReverseCrc32 {
        string: String,
        target: i32,
        #[arg(
            short,
            long,
            default_value_t = 0,
            help = "Starting value for the CRC-32 calculation"
        )]
        starting: i32,
        #[arg(short, long, default_value_t = 0)]
        min_filler_length: usize,
        #[arg(short, long, default_value_t = 10)]
        max_filler_length: usize,
        #[arg(short, long, default_value_t = DEFAULT_CHARACTER_SET.to_string())]
        character_set: String,
    },
    Unlz {
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: LzEndian,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzAlgorithm::Lzrs)]
        algorithm: LzAlgorithm,
    },
    Lz {
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: LzEndian,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzAlgorithm::Lzrs)]
        algorithm: LzAlgorithm,
    },
    Csc {},
    #[clap(alias = "xpsc")]
    ExtractPsc {
        psc: PathBuf,
        directory: PathBuf,
    },
    #[clap(alias = "cpsc")]
    CreatePsc {
        directory: PathBuf,
        psc: PathBuf,
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
        Commands::Crc {
            string,
            starting,
            algorithm,
            mode,
            format,
        } => crc::crc(string, starting, algorithm, mode, format),
        Commands::Unlz { endian, algorithm } => lz::unlz(endian, algorithm),
        Commands::Lz { endian, algorithm } => lz::lz(endian, algorithm),
        Commands::ReverseCrc32 {
            string,
            target,
            starting,
            min_filler_length,
            max_filler_length,
            character_set,
        } => reverse_crc32::reverse_crc32(
            string,
            target,
            starting,
            min_filler_length,
            max_filler_length,
            character_set,
        ),
        Commands::RoundTrip { bigfile } => round_trip::round_trip(bigfile),
        Commands::Csc {} => csc::csc(),
        Commands::ExtractPsc { psc, directory } => psc::extract_psc(psc, directory),
        Commands::CreatePsc { directory, psc } => psc::create_psc(directory, psc),
    }
}
