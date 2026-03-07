use crate::config::ConfigManager;
use crate::{log_error, log_function, log_info};
use once_cell::sync::Lazy;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use std::sync::Mutex;

const CONSOLES_DIR: &str = "consoles";

#[derive(Clone, Debug)]
pub struct SqlConsoleStorage {
    consoles_dir: PathBuf,
}

impl SqlConsoleStorage {
    pub fn new() -> io::Result<Self> {
        log_function!(new);
        let config_manager = ConfigManager::new()?;
        let consoles_dir = config_manager.get_config_dir().join(CONSOLES_DIR);

        if !consoles_dir.exists() {
            fs::create_dir_all(&consoles_dir)?;
        }

        Ok(Self { consoles_dir })
    }

    pub fn save_console_file(&self, file_name: &str, content: &str) -> io::Result<()> {
        log_function!(save_console_file, "file_name" => file_name);
        let file_path = self.consoles_dir.join(file_name);
        fs::write(file_path, content)?;
        log_info!("Successfully saved console file: {}", file_name);
        Ok(())
    }

    pub fn read_console_file(&self, file_name: &str) -> io::Result<String> {
        log_function!(read_console_file, "file_name" => file_name);
        let file_path = self.consoles_dir.join(file_name);
        if !file_path.exists() {
            log_error!("Console file not found: {}", file_name);
            return Err(Error::new(ErrorKind::NotFound, "Console file not found"));
        }
        let content = fs::read_to_string(file_path)?;
        log_info!("Successfully read console file: {}", file_name);
        Ok(content)
    }

    pub fn list_console_files(&self) -> io::Result<Vec<String>> {
        log_function!(list_console_files);
        let mut files = Vec::new();
        for entry in fs::read_dir(&self.consoles_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".sql") {
                            files.push(file_name_str.to_string());
                        }
                    }
                }
            }
        }
        log_info!(
            "Successfully listed console files. Found {} files.",
            files.len()
        );
        Ok(files)
    }

    pub fn delete_console_file(&self, file_name: &str) -> io::Result<()> {
        log_function!(delete_console_file, "file_name" => file_name);
        let file_path = self.consoles_dir.join(file_name);
        if !file_path.exists() {
            log_error!("Console file not found for deletion: {}", file_name);
            return Err(Error::new(ErrorKind::NotFound, "Console file not found"));
        }
        fs::remove_file(file_path)?;
        log_info!("Successfully deleted console file: {}", file_name);
        Ok(())
    }
}

static SQL_CONSOLE_STORAGE: Lazy<Mutex<SqlConsoleStorage>> = Lazy::new(|| {
    Mutex::new(SqlConsoleStorage::new().expect("Failed to initialize SqlConsoleStorage"))
});

pub fn get_sql_console_storage() -> &'static Mutex<SqlConsoleStorage> {
    &SQL_CONSOLE_STORAGE
}
