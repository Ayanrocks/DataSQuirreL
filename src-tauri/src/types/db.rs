use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres};

#[derive(Debug)]
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

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TableColumns {
    pub column_name: String,
    pub data_type: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TableRowCount {
    pub row_count: i64,
}
