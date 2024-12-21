use std::path::PathBuf;

use clap::Parser;
use log::{debug, info};

pub mod parse;

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
}
