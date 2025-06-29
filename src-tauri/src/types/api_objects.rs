use types::db::ConnPool;
use crate::storage::{ConnectionStorage, StoredConnection};
use crate::types;
use crate::types::db::TableSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct ApplicationState {
    pub dbpool: Mutex<Option<ConnPool>>,
    pub connection_storage: ConnectionStorage,
    pub active_connection_map: Mutex<HashMap<String, StoredConnection>>,
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
    pub columns: Vec<String>,
    pub rows: Option<Vec<Vec<T>>>,
    pub row_count: Option<String>,
    pub table_name: Option<String>,
    pub query_type: String,
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
    pub table_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableDataOffsetRequest {
    pub table_name: String,
    pub offset: u32,
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