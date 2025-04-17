use std::path::PathBuf;

use bff::bigfile::platforms::Platform;
use bff::bigfile::versions::Version;
use clap::*;
use crc::{CrcAlgorithm, CrcFormat, CrcMode};
use error::BffCliResult;
use extract::ExportStrategy;
use lz::LzEndian;

use crate::lz::LzAlgorithm;

mod cps;
mod crc;
mod create;
mod create_resource;
mod csc;
mod error;
mod extract;
mod extract_resource;
mod fat_lin;
mod info;
mod lz;
mod names;
mod psc;
mod stdio_or_path;
mod try_your_best;

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
        #[arg(short, long)]
        platform_override: Option<Platform>,
        #[arg(short, long)]
        version_override: Option<Version>,
        #[clap(value_enum)]
        #[arg(short, long, default_value_t = ExportStrategy::Binary)]
        export_strategy: ExportStrategy,
    },
    #[clap(alias = "c")]
    Create {
        directory: PathBuf,
        bigfile: PathBuf,
        #[arg(long)]
        out_names: Option<PathBuf>,
        #[arg(short, long)]
        platform_override: Option<Platform>,
        #[arg(short, long)]
        version_override: Option<Version>,
    },
    #[clap(alias = "xr")]
    ExtractResource {
        resource: PathBuf,
        directory: PathBuf,
        #[arg(long)]
        in_names: Vec<PathBuf>,
        #[arg(short, long)]
        platform_override: Option<Platform>,
        #[arg(short, long)]
        version_override: Option<Version>,
    },
    #[clap(alias = "cr")]
    CreateResource {
        directory: PathBuf,
        resource: PathBuf,
        #[arg(long)]
        out_names: Option<PathBuf>,
        #[arg(short, long)]
        platform_override: Option<Platform>,
        #[arg(short, long)]
        version_override: Option<Version>,
    },
    #[clap(alias = "t")]
    Info {
        bigfile: PathBuf,
        #[arg(long)]
        in_names: Vec<PathBuf>,
        #[arg(long)]
        out_reference_graph: Option<PathBuf>,
    },
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
    #[clap(alias = "tyb")]
    TryYourBest { path: PathBuf },
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
            platform_override,
            version_override,
            export_strategy,
        } => extract::extract(
            bigfile,
            directory,
            in_names,
            platform_override,
            version_override,
            export_strategy,
        ),
        Commands::Create {
            directory,
            bigfile,
            out_names,
            platform_override,
            version_override,
        } => create::create(
            directory,
            bigfile,
            out_names,
            platform_override,
            version_override,
        ),
        Commands::ExtractResource {
            resource,
            directory,
            in_names,
            platform_override,
            version_override,
        } => extract_resource::extract_resource(
            resource,
            directory,
            in_names,
            platform_override,
            version_override,
        ),
        Commands::CreateResource {
            directory,
            resource,
            out_names,
            platform_override,
            version_override,
        } => create_resource::create_resource(
            directory,
            resource,
            out_names,
            platform_override,
            version_override,
        ),
        Commands::Info {
            bigfile,
            in_names,
            out_reference_graph: out_dependencies,
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
        Commands::TryYourBest { path } => try_your_best::try_your_best(path),
    }
}
