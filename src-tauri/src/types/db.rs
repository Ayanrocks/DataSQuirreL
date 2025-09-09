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

pub fn sql_to_js_type(sql_type: &str) -> String {
    match sql_type.to_lowercase().as_str() {
        // String types
        "text" | "varchar" | "char" | "character" | "character varying" | "name" | "uuid" | "citext" | "xml" | "ltree" | "lquery" | "ltxtquery" | "tsvector" | "tsquery" | "bytea" | "bit" | "bit varying" | "varbit" | "inet" | "cidr" | "macaddr" | "macaddr8" | "oid" | "regproc" | "regprocedure" | "regoper" | "regoperator" | "regclass" | "regtype" | "regconfig" | "regdictionary" => "string".to_string(),
        
        // Numeric types
        "integer" | "int" | "int4" | "int8" | "smallint" | "int2" | "bigint" | "decimal" | "numeric" | "real" | "float4" | "double precision" | "float8" | "serial" | "bigserial" | "smallserial" | "serial2" | "serial4" | "serial8" | "money" => "number".to_string(),
        
        // Boolean types
        "boolean" | "bool" => "boolean".to_string(),
        
        // Date/Time types
        "timestamp" | "timestamptz" | "timestamp with time zone" | "timestamp without time zone" | "date" | "time" | "time with time zone" | "time without time zone" | "timetz" | "interval" => "Date".to_string(),
        
        // JSON types
        "json" | "jsonb" => "object".to_string(),
        
        // Geometric, Range and other complex types
        "point" | "line" | "lseg" | "box" | "path" | "polygon" | "circle" | "int4range" | "int8range" | "numrange" | "tsrange" | "tstzrange" | "daterange" => "object".to_string(),
        
        // Array types (PostgreSQL arrays)
        _ if sql_type.to_lowercase().ends_with("[]") => "array".to_string(),
        
        // Default for unknown types
        _ => "any".to_string(),
    }
}
