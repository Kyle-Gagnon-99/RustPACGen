use std::path::PathBuf;

use clap::Parser;
use config::Peripheral;
use log::{debug, info};
use parse::parse_peripheral;

pub mod config;
pub mod gen;
pub mod parse;
pub mod utils;

#[derive(Parser)]
#[command(version, about = "Rust Peripheral Access Crate Generator")]
struct CliArgs {
    /// The main entry file that contains the included or defined peripherals
    #[clap(short, long)]
    entry_file: PathBuf,

    /// The output directory where the generated crate will be saved
    #[clap(short, long)]
    output_dir: PathBuf,
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

    let project = parse::parse_project(
        cli_args
            .entry_file
            .to_str()
            .unwrap_or_else(|| panic!("Failed to convert entry file path to string")),
    );

    debug!("Parsed project: {:#?}", project);

    let mut peripherals: Vec<Peripheral> = Vec::new();

    // Loop through each includes element and parse the peripheral configuration
    // These are files that are assumed to be relative to the entry file
    // They should contain only the peripheral configuration

    // First, get the directory of the entry file
    let entry_dir = cli_args
        .entry_file
        .parent()
        .unwrap_or_else(|| panic!("Failed to get parent directory of entry file"));

    // Next, loop through each include file and parse the peripheral configuration
    // Add the parsed peripheral to the list of peripherals
    for include_file in project.includes {
        let peripheral = parse_peripheral(
            entry_dir
                .join(include_file.file)
                .to_str()
                .unwrap_or_else(|| panic!("Failed to convert include file path to string")),
        );

        debug!("Parsed peripheral: {:#?}", peripheral);

        // Add the parsed peripheral to the list of peripherals
        peripherals.push(peripheral);
    }

    // Generate the crate
    gen::generate_crate(&peripherals, cli_args.output_dir.to_str().unwrap());
}
