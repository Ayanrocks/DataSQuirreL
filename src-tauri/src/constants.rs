// ErrCodeDatabaseConnFailed - database
pub const ERR_CODE_DATABASE_CONN_FAILED: &str = "ERR_DB_CONN_FAILED";
pub const ERR_CODE_DATABASE_FETCH_TABLES_FAILED: &str = "ERR_DB_FETCH_TABLES_FAILED";
pub const ERR_CODE_DATABASE_FETCH_TABLE_DATA_FAILED: &str = "ERR_DB_FETCH_TABLE_DATA_FAILED";
pub const ERR_CODE_DATABASE_FETCH_TABLE_ROW_COUNT_FAILED: &str =
    "ERR_DB_FETCH_TABLE_ROW_COUNT_FAILED";
pub const ERR_CODE_STORAGE_FAILED: &str = "ERR_STORAGE_FAILED";
pub const ERR_CODE_INVALID_CONN_DATA: &str = "ERR_CODE_INVALID_CONN_DATA";

// PG Types
pub const BOOL: &str = "BOOL";
pub const SMALLINT: &str = "SMALLINT";
pub const SMALLSERIAL: &str = "SMALLSERIAL";
pub const INT2: &str = "INT2";
pub const INT: &str = "INT";
pub const SERIAL: &str = "SERIAL";
pub const INT4: &str = "INT4";
pub const BIGINT: &str = "BIGINT";
pub const BIGSERIAL: &str = "BIGSERIAL";
pub const INT8: &str = "INT8";
pub const REAL: &str = "REAL";
pub const FLOAT4: &str = "FLOAT4";
pub const DOUBLE_PRECISION: &str = "DOUBLE PRECISION";
pub const FLOAT8: &str = "FLOAT8";
pub const VARCHAR: &str = "VARCHAR";
pub const CHAR: &str = "CHAR(N)";
pub const TEXT: &str = "TEXT";
pub const CITEXT: &str = "CITEXT";
pub const NAME: &str = "NAME";
pub const BYTEA: &str = "BYTEA";
pub const TIMESTAMPTZ: &str = "TIMESTAMPTZ";
pub const TIMESTAMP: &str = "TIMESTAMP";
pub const DATE: &str = "DATE";
pub const TIME: &str = "TIME";
pub const UUID: &str = "UUID";
pub const INET: &str = "INET";
pub const CIDR: &str = "CIDR";
pub const JSON: &str = "JSON";
pub const JSONB: &str = "JSONB";

pub const INITIAL_PAGE_SIZE: u32 = 40;

// Query Types
pub const QUERY_TYPE_FETCH_TABLES: &str = "FETCH_TABLES";
pub const QUERY_TYPE_FETCH_INITIAL_TABLE_DATA: &str = "FETCH_INITIAL_TABLE_DATA";
pub const QUERY_TYPE_FETCH_OFFSET_TABLE_DATA: &str = "FETCH_OFFSET_TABLE_DATA";
pub const QUERY_TYPE_CUSTOM: &str = "custom";

pub const APP_NAME: &str = "DataSquirrel";

// Datbase types
pub const POSTGRES_DATABASE_TYPE: &str = "postgresql";