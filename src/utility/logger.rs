use fern;
use chrono;
use std::env;
use log::LevelFilter;

pub fn setup_logger() -> Result<(), fern::InitError> {
    dotenv::dotenv().ok();

    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_level_filter = match log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info, // Default
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level_filter)
        .chain(std::io::stdout()) // Only log to stdout
        .apply()?;
    Ok(())
}
