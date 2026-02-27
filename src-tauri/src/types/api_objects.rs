use crate::cache::CacheDB;
use crate::sql_console_storage::SqlConsoleStorage;
use crate::storage::{ConnectionStorage, StoredConnection};
use crate::types;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use types::db::ConnPool;

pub struct ApplicationState {
    pub dbpool: Mutex<Option<ConnPool>>,
    pub connection_storage: ConnectionStorage,
    pub active_connection_map: Mutex<HashMap<String, StoredConnection>>,
    pub sql_console_storage: SqlConsoleStorage,
    pub sqlite_db: CacheDB,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBConnectionRequest {
    pub id: String,
    pub conn_name: String,
    pub host_name: String,
    pub database_name: String,
    pub database_type: String,
    pub port: i32,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableData<T> {
    pub columns: Vec<(String, String)>,
    pub rows: Option<Vec<Vec<T>>>,
    pub row_count: Option<String>,
    pub table_name: Option<String>,
    pub query_type: String,
    pub primary_keys: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPCResponse<T> {
    pub status: u16,
    pub error_code: Option<String>,
    pub sys_err: Option<String>,
    pub frontend_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableDataRequest {
    pub database_name: String,
    pub schema_name: String,
    pub table_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableDataOffsetRequest {
    pub database_name: String,
    pub schema_name: String,
    pub table_name: String,
    pub offset: u32,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    pub connection_data: HashMap<String, String>,
    pub dashboard_data: Vec<SchemaData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaData {
    #[serde(rename = "entityType")]
    pub entity_type: String,
    #[serde(rename = "entityName")]
    pub entity_name: String,
    #[serde(rename = "isExpanded")]
    pub is_expanded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<SchemaData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardDataRequest {
    pub connection_window_label: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionChange {
    pub r#type: String, // "INSERT" | "UPDATE" | "DELETE"
    pub row_index: usize,
    pub primary_keys: Option<HashMap<String, String>>,
    pub original_row: Option<HashMap<String, String>>,
    pub new_values: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitTransactionRequest {
    pub database_name: String,
    pub schema_name: String,
    pub table_name: String,
    pub changes: Vec<TransactionChange>,
}
