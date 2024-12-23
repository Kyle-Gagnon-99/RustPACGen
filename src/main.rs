use std::path::PathBuf;

use clap::Parser;
use config::parse_config;
use log::{debug, info};

pub mod config;
pub mod gen;
pub mod parse;

/// Constants for default values for the versions for the crates
pub mod defaults {
    pub const BIT_FIELD_VERSION: &str = "0.10.2";
    pub const VOLATILE_REGISTER_VERSION: &str = "0.2.2";
}

#[derive(Parser)]
#[command(version, about = "Rust Peripheral Access Crate Generator")]
struct CliArgs {
    /// The main entry file that contains the included or defined peripherals
    #[clap(short, long)]
    entry_file: PathBuf,

    /// The output directory where the generated crate will be saved
    #[clap(short, long)]
    output_dir: PathBuf,

    /// The name of the generated crate. It should follow the naming conventions of a Rust crate and match the name of the output directory typically.
    #[clap(short, long)]
    crate_name: String,

    /// The configuration file that dictates the versions of the dependencies that is used by the generated crate.
    /// This is optional and if not provided, the latest versions of the dependencies will be used.
    #[clap(long)]
    config_file: Option<PathBuf>,
}

/// Initalizes the logger to be used by the application
fn init_logger() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn main() {
    init_logger();

    let cli_args = CliArgs::parse();

    info!("Starting Rust Peripheral Access Crate Generator");
    debug!("Entry file: {:?}", cli_args.entry_file);
    debug!("Output directory: {:?}", cli_args.output_dir);

    // Parse out the configuration file
    let config_file = parse_config(&cli_args.config_file);
    debug!("Parsed configuration file: {:#?}", config_file);

    let peripheral_access = parse::parse_input(&cli_args.entry_file);
    debug!("Parsed peripheral access: {:#?}", peripheral_access);

    // Generate the peripheral access crate code
    match peripheral_access {
        Ok(pac) => {
            gen::generate_pac_code(
                &cli_args.output_dir,
                &cli_args.crate_name,
                &pac,
                &config_file,
            );
        }
        Err(e) => {
            eprintln!("Error parsing input file: {}", e);
        }
    }
}
