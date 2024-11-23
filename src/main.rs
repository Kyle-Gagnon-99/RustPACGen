use log::info;

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
    info!("Starting Rust Peripheral Access Crate Generator");
}
