use crate::config::ConfigManager;
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

    pub fn with_dir(consoles_dir: PathBuf) -> io::Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn setup_storage() -> SqlConsoleStorage {
        let temp_dir = env::temp_dir();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_dir = temp_dir.join(format!("datasquirrel_test_consoles_{}", timestamp));
        SqlConsoleStorage::with_dir(test_dir).unwrap()
    }

    #[test]
    fn test_save_and_read_console_file() {
        let storage = setup_storage();
        storage
            .save_console_file("test1.sql", "SELECT * FROM test;")
            .unwrap();

        let content = storage.read_console_file("test1.sql").unwrap();
        assert_eq!(content, "SELECT * FROM test;");
    }

    #[test]
    fn test_read_non_existent_file() {
        let storage = setup_storage();
        let result = storage.read_console_file("missing.sql");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_list_console_files() {
        let storage = setup_storage();
        storage.save_console_file("file1.sql", "query 1").unwrap();
        storage.save_console_file("file2.sql", "query 2").unwrap();
        storage
            .save_console_file("file3.txt", "not a query")
            .unwrap();

        let mut files = storage.list_console_files().unwrap();
        files.sort();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0], "file1.sql");
        assert_eq!(files[1], "file2.sql");
    }

    #[test]
    fn test_delete_console_file() {
        let storage = setup_storage();
        storage.save_console_file("todelete.sql", "data").unwrap();

        assert!(storage.read_console_file("todelete.sql").is_ok());

        storage.delete_console_file("todelete.sql").unwrap();

        assert!(storage.read_console_file("todelete.sql").is_err());
    }
}
