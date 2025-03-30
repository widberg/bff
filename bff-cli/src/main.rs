use std::path::PathBuf;

use clap::*;
use crc::{CrcAlgorithm, CrcFormat, CrcMode};
use error::BffCliResult;
use lz::LzEndian;
use reverse_crc32::DEFAULT_CHARACTER_SET;

use crate::lz::LzAlgorithm;

mod cps;
mod crc;
mod create;
mod csc;
mod error;
mod extract;
mod fat_lin;
mod info;
mod lz;
mod names;
mod psc;
mod reverse_crc32;
mod round_trip;
mod stdio_or_path;

use shadow_rs::shadow;

use crate::names::Wordlist;
use crate::psc::PscAlgorithm;
use crate::stdio_or_path::StdioOrPath;

shadow!(build);

#[derive(Subcommand)]
enum Commands {
    #[clap(alias = "x")]
    Extract {
        bigfile: PathBuf,
        directory: PathBuf,
        #[arg(long)]
        in_names: Vec<PathBuf>,
    },
    #[clap(alias = "c")]
    Create {
        directory: PathBuf,
        bigfile: PathBuf,
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
        #[arg(long)]
        out_reference_map: Option<PathBuf>,
    },
    #[clap(alias = "rt")]
    RoundTrip { bigfile: PathBuf },
    Names {
        bigfile: Option<PathBuf>,
        #[clap(value_enum)]
        #[arg(short, long)]
        wordlist: Option<Wordlist>,
        #[arg(long)]
        in_names: Vec<PathBuf>,
        #[arg(long)]
        out_names: Option<PathBuf>,
    },
    Crc {
        string: Option<String>,
        #[arg(
            short,
            long,
            default_value_t,
            help = "Starting value for the CRC calculation"
        )]
        starting: i64,
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
        compressed: StdioOrPath,
        uncompressed: StdioOrPath,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: LzEndian,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzAlgorithm::Lzrs)]
        algorithm: LzAlgorithm,
    },
    Lz {
        uncompressed: StdioOrPath,
        compressed: StdioOrPath,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: LzEndian,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzAlgorithm::Lzrs)]
        algorithm: LzAlgorithm,
    },
    Csc {
        input: StdioOrPath,
        output: StdioOrPath,
        #[arg(
            short,
            long,
            default_value_t = 255,
            help = "If the default value of 255 does not work, try 252"
        )]
        key: u8,
    },
    #[clap(alias = "xpsc")]
    ExtractPsc {
        psc: PathBuf,
        directory: PathBuf,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = PscAlgorithm::Lz4)]
        algorithm: PscAlgorithm,
    },
    #[clap(alias = "cpsc")]
    CreatePsc {
        directory: PathBuf,
        psc: PathBuf,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = PscAlgorithm::Lz4)]
        algorithm: PscAlgorithm,
    },
    #[clap(alias = "xcps")]
    ExtractCps {
        cps: PathBuf,
        directory: PathBuf,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: LzEndian,
    },
    #[clap(alias = "ccps")]
    CreateCps {
        directory: PathBuf,
        cps: PathBuf,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = LzEndian::Little)]
        endian: LzEndian,
        #[arg(short, long)]
        unencrypted: bool,
    },
    #[clap(alias = "xfl")]
    ExtractFatLin {
        fat: PathBuf,
        lin: PathBuf,
        directory: PathBuf,
    },
    #[clap(alias = "cfl")]
    CreateFatLin {
        directory: PathBuf,
        fat: PathBuf,
        lin: PathBuf,
    },
}

#[derive(Parser)]
#[command(author, version, long_version = build::CLAP_LONG_VERSION, about, long_about = None)]
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
        } => extract::extract(bigfile, directory, in_names),
        Commands::Create {
            directory,
            bigfile,
            in_names,
            out_names,
        } => create::create(directory, bigfile, in_names, out_names),
        Commands::Info {
            bigfile,
            in_names,
            out_reference_map: out_dependencies,
        } => info::info(bigfile, in_names, out_dependencies),
        Commands::Names {
            bigfile,
            wordlist,
            in_names,
            out_names,
        } => names::names(bigfile, wordlist, in_names, out_names),
        Commands::Crc {
            string,
            starting,
            algorithm,
            mode,
            format,
        } => crc::crc(string, starting, algorithm, mode, format),
        Commands::Unlz {
            compressed,
            uncompressed,
            endian,
            algorithm,
        } => lz::unlz(compressed, uncompressed, endian, algorithm),
        Commands::Lz {
            uncompressed,
            compressed,
            endian,
            algorithm,
        } => lz::lz(uncompressed, compressed, endian, algorithm),
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
        Commands::Csc { input, output, key } => csc::csc(input, output, key),
        Commands::ExtractPsc {
            psc,
            directory,
            algorithm,
        } => psc::extract_psc(psc, directory, algorithm),
        Commands::CreatePsc {
            directory,
            psc,
            algorithm,
        } => psc::create_psc(directory, psc, algorithm),
        Commands::ExtractCps {
            cps,
            directory,
            endian,
        } => cps::extract_cps(cps, directory, endian),
        Commands::CreateCps {
            directory,
            cps,
            endian,
            unencrypted,
        } => cps::create_cps(directory, cps, endian, unencrypted),
        Commands::ExtractFatLin {
            fat,
            lin,
            directory,
        } => fat_lin::extract_fat_lin(fat, lin, directory),
        Commands::CreateFatLin {
            directory,
            fat,
            lin,
        } => fat_lin::create_fat_lin(directory, fat, lin),
    }
}
