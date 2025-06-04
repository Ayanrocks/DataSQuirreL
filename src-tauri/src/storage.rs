use crate::config::{ConfigType, get_config_manager};
use crate::log_function;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tauri::AppHandle;

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

pub struct ConnectionStorage;

impl ConnectionStorage {
    pub fn new() -> Self {
        log_function!(new);
        ConnectionStorage
    }

    pub fn save_connection(
        &self,
        app: &AppHandle,
        conn: &StoredConnection,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        log_function!(save_connection);
        let connections = self.get_all_connections(app)?;
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

        config_manager.write_config(
            &ConfigType::Connections,
            &serde_json::to_string(&updated_connections)?,
        )?;

        let entry = Entry::new("datasquirrel", &conn.id)?;
        entry.set_password(password)?;

        Ok(())
    }

    pub fn get_all_connections(
        &self,
        app: &AppHandle,
    ) -> Result<Vec<StoredConnection>, Box<dyn Error>> {
        log_function!(get_all_connections);
        let config_manager = get_config_manager().lock().unwrap();
        match config_manager.read_config::<Vec<StoredConnection>>(&ConfigType::Connections) {
            Ok(connections) => Ok(connections),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn get_connection(
        &self,
        app: &AppHandle,
        conn_name: &str,
    ) -> Result<Option<StoredConnection>, Box<dyn Error>> {
        log_function!(get_connection, "app" => app, "conn_name" => conn_name);
        let connections = self.get_all_connections(app)?;
        Ok(connections.into_iter().find(|c| c.conn_name == conn_name))
    }

    pub fn get_password(&self, conn_name: &str) -> Result<String, Box<dyn Error>> {
        log_function!(get_password);
        let entry = Entry::new("datasquirrel", conn_name)?;
        Ok(entry.get_password()?)
    }

    pub fn delete_connection(
        &self,
        app: &AppHandle,
        conn_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        log_function!(delete_connection, "app" => app, "conn_name" => conn_name);
        let connections = self.get_all_connections(app)?;
        let updated_connections: Vec<StoredConnection> = connections
            .into_iter()
            .filter(|c| c.conn_name != conn_name)
            .collect();

        let config_manager = get_config_manager().lock().unwrap();
        config_manager.write_config(
            &ConfigType::Connections,
            &serde_json::to_string(&updated_connections)?,
        )?;

        let entry = Entry::new("datasquirrel", conn_name)?;
        entry.delete_credential()?;

        Ok(())
    }
}
