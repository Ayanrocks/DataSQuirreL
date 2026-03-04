use super::types_mapper::DbTypeMapper;

pub struct PostgresMapper;

impl DbTypeMapper for PostgresMapper {
    fn sql_to_js_type(&self, sql_type: &str) -> String {
        match sql_type.to_lowercase().as_str() {
            // String types
            "text" | "varchar" | "char" | "character" | "character varying" | "name" | "uuid"
            | "citext" | "xml" | "ltree" | "lquery" | "ltxtquery" | "tsvector" | "tsquery"
            | "bytea" | "bit" | "bit varying" | "varbit" | "inet" | "cidr" | "macaddr"
            | "macaddr8" | "oid" | "regproc" | "regprocedure" | "regoper" | "regoperator"
            | "regclass" | "regtype" | "regconfig" | "regdictionary" => "string".to_string(),

            // Numeric types
            "integer" | "int" | "int4" | "int8" | "smallint" | "int2" | "bigint" | "decimal"
            | "numeric" | "real" | "float4" | "double precision" | "float8" | "serial"
            | "bigserial" | "smallserial" | "serial2" | "serial4" | "serial8" | "money" => {
                "number".to_string()
            }

            // Boolean types
            "boolean" | "bool" => "boolean".to_string(),

            // Date/Time types
            "timestamp"
            | "timestamptz"
            | "timestamp with time zone"
            | "timestamp without time zone"
            | "date"
            | "time"
            | "time with time zone"
            | "time without time zone"
            | "timetz"
            | "interval" => "Date".to_string(),

            // JSON types
            "json" | "jsonb" => "object".to_string(),

            // Geometric, Range and other complex types
            "point" | "line" | "lseg" | "box" | "path" | "polygon" | "circle" | "int4range"
            | "int8range" | "numrange" | "tsrange" | "tstzrange" | "daterange" => {
                "object".to_string()
            }

            // Array types (PostgreSQL arrays)
            _ if sql_type.to_lowercase().ends_with("[]") => "array".to_string(),

            // Default for unknown types
            _ => "any".to_string(),
        }
    }

    fn cast_to_text_expr(&self, column_name: &str, _raw_type: &str) -> String {
        format!("\"{}\"::text AS \"{}\"", column_name, column_name)
    }
}
