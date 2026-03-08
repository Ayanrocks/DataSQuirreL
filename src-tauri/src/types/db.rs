use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConnPool {
    pub conn_name: String,
    pub db_name: String,
    pub dsn: String,
    pub pool: sqlx::Pool<Postgres>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_type: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TableColumns {
    pub column_name: String,
    pub data_type: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TableRowCount {
    pub row_count: i64,
}

/// Result of executing a raw SQL query via the console.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawQueryResult {
    pub columns: Vec<String>,
    pub rows: Option<Vec<Vec<String>>>,
    pub row_count: i64,
    pub execution_time_ms: u64,
    pub is_select: bool,
}
