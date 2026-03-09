#![macro_use]

use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, prelude::*};

use crate::constants::APP_NAME;

static LOGGER: OnceCell<()> = OnceCell::new();
static STDOUT_GUARD: OnceCell<tracing_appender::non_blocking::WorkerGuard> = OnceCell::new();

fn get_log_dir() -> String {
    // make the app name lowercase
    let log_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(format!(".{}", APP_NAME.to_lowercase()))
        .join("logs");
    log_dir.to_string_lossy().to_string()
}

/// Initialize the logging system with both file and stdout output
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    LOGGER.get_or_try_init(|| -> Result<(), Box<dyn std::error::Error>> {
        let log_dir = get_log_dir();
        print!("LOGGING DIR: {log_dir:?}");

        // Create log directory if it doesn't exist
        std::fs::create_dir_all(&log_dir)?;

        // Configure file appender with rotation
        let file_appender =
            RollingFileAppender::new(Rotation::DAILY, log_dir, format!("{APP_NAME}.log"));

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

/// Macro to log function entry and exit with optional arguments
#[macro_export]
macro_rules! log_function {
    // Pattern 1: Function identifier (converts to string)
    ($fn_name:ident) => {{
        let function_name = stringify!($fn_name);
        tracing::info!("Entering function: {}", function_name);
        let _guard = tracing::span!(tracing::Level::INFO, "function", name = function_name).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};

    // Pattern 2: Function name as string literal
    ($fn_name:expr) => {{
        let function_name = $fn_name;
        tracing::info!("Entering function: {}", function_name);
        let _guard = tracing::span!(tracing::Level::INFO, "function", name = function_name).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};

    // Pattern 3: Function identifier with arguments (using format strings)
    ($fn_name:ident, $($arg_name:expr => $arg_value:expr),+ $(,)?) => {{
        let function_name = stringify!($fn_name);

        // Build argument string without requiring Debug trait
        let mut args_str = String::new();
        $(
            if !args_str.is_empty() {
                args_str.push_str(", ");
            }
            args_str.push_str(&format!("{} = {}", $arg_name,
                // Try to display the value, fallback to type name if not displayable
                std::any::type_name_of_val(&$arg_value)
            ));
        )+

        tracing::info!("Entering function: {} with args: [{}]", function_name, args_str);
        let _guard = tracing::span!(
            tracing::Level::INFO,
            "function",
            name = function_name,
            args = args_str.as_str()
        ).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};

    // Pattern 4: Function name with arguments (using format strings)
    ($fn_name:expr, $($arg_name:expr => $arg_value:expr),+ $(,)?) => {{
        let function_name = $fn_name;

        // Build argument string without requiring Debug trait
        let mut args_str = String::new();
        $(
            if !args_str.is_empty() {
                args_str.push_str(", ");
            }
            args_str.push_str(&format!("{} = {}", $arg_name,
                // Try to display the value, fallback to type name if not displayable
                std::any::type_name_of_val(&$arg_value)
            ));
        )+

        tracing::info!("Entering function: {} with args: [{}]", function_name, args_str);
        let _guard = tracing::span!(
            tracing::Level::INFO,
            "function",
            name = function_name,
            args = args_str.as_str()
        ).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};

    // Pattern 5: Function identifier with arguments that implement Display
    ($fn_name:ident, display: $($arg_name:expr => $arg_value:expr),+ $(,)?) => {{
        let function_name = stringify!($fn_name);

        // Build argument string using Display trait
        let mut args_str = String::new();
        $(
            if !args_str.is_empty() {
                args_str.push_str(", ");
            }
            args_str.push_str(&format!("{} = {}", $arg_name, $arg_value));
        )+

        tracing::info!("Entering function: {} with args: [{}]", function_name, args_str);
        let _guard = tracing::span!(
            tracing::Level::INFO,
            "function",
            name = function_name,
            args = args_str.as_str()
        ).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};

    // Pattern 6: Function name with arguments that implement Display
    ($fn_name:expr, display: $($arg_name:expr => $arg_value:expr),+ $(,)?) => {{
        let function_name = $fn_name;

        // Build argument string using Display trait
        let mut args_str = String::new();
        $(
            if !args_str.is_empty() {
                args_str.push_str(", ");
            }
            args_str.push_str(&format!("{} = {}", $arg_name, $arg_value));
        )+

        tracing::info!("Entering function: {} with args: [{}]", function_name, args_str);
        let _guard = tracing::span!(
            tracing::Level::INFO,
            "function",
            name = function_name,
            args = args_str.as_str()
        ).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};

    // Pattern 7: Function identifier with arguments that implement Debug
    ($fn_name:ident, debug: $($arg_name:expr => $arg_value:expr),+ $(,)?) => {{
        let function_name = stringify!($fn_name);

        // Build argument string using Debug trait
        let mut args_str = String::new();
        $(
            if !args_str.is_empty() {
                args_str.push_str(", ");
            }
            args_str.push_str(&format!("{} = {:?}", $arg_name, $arg_value));
        )+

        tracing::info!("Entering function: {} with args: [{}]", function_name, args_str);
        let _guard = tracing::span!(
            tracing::Level::INFO,
            "function",
            name = function_name,
            args = args_str.as_str()
        ).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};

    // Pattern 8: Function name with arguments that implement Debug
    ($fn_name:expr, debug: $($arg_name:expr => $arg_value:expr),+ $(,)?) => {{
        let function_name = $fn_name;

        // Build argument string using Debug trait
        let mut args_str = String::new();
        $(
            if !args_str.is_empty() {
                args_str.push_str(", ");
            }
            args_str.push_str(&format!("{} = {:?}", $arg_name, $arg_value));
        )+

        tracing::info!("Entering function: {} with args: [{}]", function_name, args_str);
        let _guard = tracing::span!(
            tracing::Level::INFO,
            "function",
            name = function_name,
            args = args_str.as_str()
        ).entered();

        // Create a defer-like mechanism for exit logging
        struct ExitLogger<'a> {
            function_name: &'a str,
        }

        impl<'a> Drop for ExitLogger<'a> {
            fn drop(&mut self) {
                tracing::info!("Exiting function: {}", self.function_name);
            }
        }

        let _exit_logger = ExitLogger { function_name };
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_log_dir() {
        let dir = get_log_dir();
        assert!(!dir.is_empty());
        assert!(dir.contains(&APP_NAME.to_lowercase()));
    }

    #[test]
    fn test_macros_compile() {
        // We just ensure they compile. Since there's no initialized logger in this isolated
        // test, it will just be a no-op at runtime (or log to tracing's default sink).
        log_error!("Test error");
        log_warn!("Test warning");
        log_info!("Test info");
        log_debug!("Test debug");
        log_trace!("Test trace");

        log_function!(test_macros_compile);
        log_function!("custom_name");
        log_function!(test_macros_compile, "arg1" => 42);
    }
}
