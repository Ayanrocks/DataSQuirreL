use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, prelude::*};

static LOGGER: OnceCell<()> = OnceCell::new();
const APP_NAME: &str = "dataSquirrel";
static STDOUT_GUARD: OnceCell<tracing_appender::non_blocking::WorkerGuard> = OnceCell::new();

fn get_log_dir() -> String {
    // make the app name lowercase
    let log_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(format!(".{}", APP_NAME.to_lowercase()))
        .join("logs");
    return log_dir.to_string_lossy().to_string();
}

/// Initialize the logging system with both file and stdout output
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    LOGGER.get_or_try_init(|| -> Result<(), Box<dyn std::error::Error>> {
        let log_dir = get_log_dir();
        print!("LOGGING DIR: {:?}", log_dir);

        // Create log directory if it doesn't exist
        std::fs::create_dir_all(&log_dir)?;

        // Configure file appender with rotation
        let file_appender =
            RollingFileAppender::new(Rotation::DAILY, log_dir, format!("{}.log", APP_NAME));

        // Configure stdout appender
        let (stdout_appender, guard) = tracing_appender::non_blocking(std::io::stdout());
        STDOUT_GUARD.set(guard).expect("Failed to set stdout guard");

        // Create the subscriber
        let subscriber = tracing_subscriber::registry()
            .with(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(stdout_appender)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_level(true)
                    .with_ansi(true),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file_appender)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_level(true)
                    .with_ansi(false)
                    .json(),
            );

        // Set the global subscriber
        tracing::subscriber::set_global_default(subscriber)?;

        Ok(())
    })?;

    Ok(())
}

/// Helper macro to log with context
#[macro_export]
macro_rules! log_with_context {
    ($level:expr, $($arg:tt)*) => {{
        let context = format!("[{}:{}]", file!(), line!());
        match $level {
            tracing::Level::ERROR => tracing::error!("{} {}", context, format!($($arg)*)),
            tracing::Level::WARN => tracing::warn!("{} {}", context, format!($($arg)*)),
            tracing::Level::INFO => tracing::info!("{} {}", context, format!($($arg)*)),
            tracing::Level::DEBUG => tracing::debug!("{} {}", context, format!($($arg)*)),
            tracing::Level::TRACE => tracing::trace!("{} {}", context, format!($($arg)*)),
        }
    }};
}

/// Helper macros for different log levels
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => { log_with_context!(tracing::Level::ERROR, $($arg)*) };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => { log_with_context!(tracing::Level::WARN, $($arg)*) };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => { log_with_context!(tracing::Level::INFO, $($arg)*) };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => { log_with_context!(tracing::Level::DEBUG, $($arg)*) };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => { log_with_context!(tracing::Level::TRACE, $($arg)*) };
}

/// Macro to log function entry and exit
#[macro_export]
macro_rules! log_function {
    ($($arg:tt)*) => {{
        let function_name = stringify!($($arg)*);
        tracing::info!("Entering function: {}", function_name);
        let _guard = tracing::span!(tracing::Level::INFO, "function", name = function_name).entered();
        tracing::info!("Exiting function: {}", function_name);
    }};
}
