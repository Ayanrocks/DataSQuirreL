use crate::config::{ConfigType, get_config_manager};
use crate::constants::APP_NAME;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredConnection {
    pub id: String,
    pub conn_name: String,
    pub host_name: String,
    pub database_name: String,
    pub database_type: String,
    pub port: i32,
    pub user_name: String,
    // Password is not stored here, it's stored in the keyring
}

#[derive(Clone, Debug)]
pub struct ConnectionStorage {
    mock_dir: Option<PathBuf>,
}

impl ConnectionStorage {
    pub fn new() -> Self {
        log_function!(new);
        ConnectionStorage { mock_dir: None }
    }

    #[cfg(test)]
    pub fn with_dir(dir: PathBuf) -> Self {
        ConnectionStorage {
            mock_dir: Some(dir),
        }
    }

    fn read_connections(&self) -> Result<Vec<StoredConnection>, Box<dyn Error>> {
        if let Some(dir) = &self.mock_dir {
            let config_manager = crate::config::ConfigManager::with_dir(dir.clone())?;
            match config_manager.read_config::<Vec<StoredConnection>>(&ConfigType::Connections) {
                Ok(c) => Ok(c),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            let config_manager = get_config_manager().lock().unwrap();
            match config_manager.read_config::<Vec<StoredConnection>>(&ConfigType::Connections) {
                Ok(c) => Ok(c),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
                Err(e) => Err(Box::new(e)),
            }
        }
    }

    fn write_connections(&self, conns: &[StoredConnection]) -> Result<(), Box<dyn Error>> {
        if let Some(dir) = &self.mock_dir {
            let config_manager = crate::config::ConfigManager::with_dir(dir.clone())?;
            config_manager.write_config(&ConfigType::Connections, &conns)?;
            Ok(())
        } else {
            let config_manager = get_config_manager().lock().unwrap();
            config_manager.write_config(&ConfigType::Connections, &conns)?;
            Ok(())
        }
    }

    pub fn save_connection(
        &self,
        conn: &StoredConnection,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        log_function!(save_connection);
        let connections = self.get_all_connections()?;
        let mut updated_connections = connections.clone();

        if let Some(index) = connections.iter().position(|c| c.id == conn.id) {
            updated_connections[index] = conn.clone();
        } else {
            updated_connections.push(conn.clone());
        }

        if let Some(dir) = &self.mock_dir {
            // Mock keyring logic: persist password to a mock file keyed by connection id
            let mock_keyring_path = dir.join(format!("{}.mock_key", conn.id));
            std::fs::write(&mock_keyring_path, password)?;
            self.write_connections(&updated_connections)?;
            return Ok(());
        }

        let entry = Entry::new(APP_NAME, &conn.id)?;
        match entry.set_password(password) {
            Ok(_) => {
                log_info!("Successfully saved to keyring");
                // Only write to the config file after successfully persisting to keyring
                self.write_connections(&updated_connections)?;
                Ok(())
            }
            Err(keyring::Error::NoEntry) => {
                log_error!("Keyring service not available");
                Err(Box::new(keyring::Error::NoEntry))
            }
            Err(e) => {
                log_error!("Other keyring error: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn get_all_connections(&self) -> Result<Vec<StoredConnection>, Box<dyn Error>> {
        log_function!(get_all_connections);
        self.read_connections()
    }

    #[allow(dead_code)]
    pub fn get_connection(&self, id: &str) -> Result<Option<StoredConnection>, Box<dyn Error>> {
        log_function!(get_connection, "id" => id);
        let connections = self.get_all_connections()?;
        Ok(connections.into_iter().find(|c| c.id == id))
    }

    #[allow(dead_code)]
    pub fn get_password(&self, conn_id: &str) -> Result<String, Box<dyn Error>> {
        log_function!(get_password);
        if let Some(dir) = &self.mock_dir {
            let mock_keyring_path = dir.join(format!("{}.mock_key", conn_id));
            if mock_keyring_path.exists() {
                let p = std::fs::read_to_string(&mock_keyring_path)?;
                return Ok(p);
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Mock password not found",
                )));
            }
        }
        let entry = Entry::new(APP_NAME, conn_id)?;
        Ok(entry.get_password()?)
    }

    pub fn delete_connection(
        &self,
        conn_name: &str,
        project_id: &str,
    ) -> Result<String, Box<dyn Error>> {
        log_function!(delete_connection, "conn_name" => conn_name);
        let connections = self.get_all_connections()?;
        let updated_connections: Vec<StoredConnection> = connections
            .into_iter()
            .filter(|c| !(c.conn_name == conn_name && project_id == c.id))
            .collect();

        if let Some(dir) = &self.mock_dir {
            let mock_keyring_path = dir.join(format!("{}.mock_key", project_id));
            if mock_keyring_path.exists() {
                std::fs::remove_file(mock_keyring_path)?;
            }
            self.write_connections(&updated_connections)?;
            return Ok("Connection deleted successfully".to_string());
        }

        let entry = Entry::new(APP_NAME, project_id)?;
        entry.delete_credential()?;
        // Delete from JSON only after credential removed successfully
        self.write_connections(&updated_connections)?;

        Ok("Connection deleted successfully".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn setup_storage() -> ConnectionStorage {
        let temp_dir = env::temp_dir();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_dir = temp_dir.join(format!("datasquirrel_test_storage_{}", timestamp));
        ConnectionStorage::with_dir(test_dir)
    }

    fn mock_conn(name: &str, id: &str) -> StoredConnection {
        StoredConnection {
            id: id.to_string(),
            conn_name: name.to_string(),
            host_name: "localhost".to_string(),
            database_name: "postgres".to_string(),
            database_type: "postgresql".to_string(),
            port: 5432,
            user_name: "admin".to_string(),
        }
    }

    #[test]
    fn test_save_and_get_all_connections() {
        let storage = setup_storage();
        assert_eq!(storage.get_all_connections().unwrap().len(), 0);

        let conn1 = mock_conn("test1", "id1");
        storage.save_connection(&conn1, "pass").unwrap();

        let conns = storage.get_all_connections().unwrap();
        assert_eq!(conns.len(), 1);
        assert_eq!(conns[0].conn_name, "test1");

        let conn2 = mock_conn("test2", "id2");
        storage.save_connection(&conn2, "pass").unwrap();

        let conns = storage.get_all_connections().unwrap();
        assert_eq!(conns.len(), 2);
    }

    #[test]
    fn test_update_existing_connection() {
        let storage = setup_storage();
        let mut conn1 = mock_conn("test1", "id1");
        storage.save_connection(&conn1, "pass").unwrap();

        conn1.host_name = "127.0.0.1".to_string();
        storage.save_connection(&conn1, "pass").unwrap();

        let conns = storage.get_all_connections().unwrap();
        assert_eq!(conns.len(), 1);
        assert_eq!(conns[0].host_name, "127.0.0.1");
    }

    #[test]
    fn test_get_specific_connection() {
        let storage = setup_storage();
        let conn1 = mock_conn("target_conn", "id1");
        storage.save_connection(&conn1, "pass").unwrap();

        let found = storage.get_connection("id1").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "id1");

        let not_found = storage.get_connection("missing_id");
        assert!(not_found.unwrap().is_none());
    }

    #[test]
    fn test_delete_connection() {
        let storage = setup_storage();
        storage
            .save_connection(&mock_conn("del_conn", "id1"), "pass")
            .unwrap();
        storage
            .save_connection(&mock_conn("keep_conn", "id2"), "pass")
            .unwrap();

        assert_eq!(storage.get_all_connections().unwrap().len(), 2);

        storage.delete_connection("del_conn", "id1").unwrap();

        let conns = storage.get_all_connections().unwrap();
        assert_eq!(conns.len(), 1);
        assert_eq!(conns[0].conn_name, "keep_conn");
    }
}
