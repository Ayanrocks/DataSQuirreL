use crate::config::{ConfigType, get_config_manager};
use crate::constants::APP_NAME;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
pub struct ConnectionStorage;

impl ConnectionStorage {
    pub fn new() -> Self {
        log_function!(new);
        ConnectionStorage
    }

    pub fn save_connection(
        &self,
        conn: &StoredConnection,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        log_function!(save_connection);
        let connections = self.get_all_connections()?;
        let mut updated_connections = connections.clone();

        if let Some(index) = connections
            .iter()
            .position(|c| c.conn_name == conn.conn_name)
        {
            updated_connections[index] = conn.clone();
        } else {
            updated_connections.push(conn.clone());
        }

        // fetch config_manager
        let config_manager = get_config_manager().lock().unwrap();

        config_manager.write_config(&ConfigType::Connections, &updated_connections)?;

        let entry = Entry::new(APP_NAME, &conn.id)?;
        // entry.set_password(password)?;
        // Check if we can access the keyring first
        match entry.set_password(password) {
            Ok(_) => {
                log_info!("Successfully saved to keyring");
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
        let config_manager = get_config_manager().lock().unwrap();
        match config_manager.read_config::<Vec<StoredConnection>>(&ConfigType::Connections) {
            Ok(connections) => Ok(connections),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(Box::new(e)),
        }
    }

    #[allow(dead_code)]
    pub fn get_connection(
        &self,
        conn_name: &str,
    ) -> Result<Option<StoredConnection>, Box<dyn Error>> {
        log_function!(get_connection, "conn_name" => conn_name);
        let connections = self.get_all_connections()?;
        Ok(connections.into_iter().find(|c| c.conn_name == conn_name))
    }

    #[allow(dead_code)]
    pub fn get_password(&self, conn_name: &str) -> Result<String, Box<dyn Error>> {
        log_function!(get_password);
        let entry = Entry::new(APP_NAME, conn_name)?;
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
            .filter(|c| c.conn_name != conn_name && project_id != c.id)
            .collect();

        let config_manager = get_config_manager().lock().unwrap();
        config_manager.write_config(&ConfigType::Connections, &updated_connections)?;

        let entry = Entry::new(APP_NAME, project_id)?;
        entry.delete_credential()?;

        Ok("Connection deleted successfully".to_string())
    }
}
