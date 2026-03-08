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
        format!("\"{column_name}\"::text AS \"{column_name}\"")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_to_js_type() {
        let mapper = PostgresMapper;

        // String types
        assert_eq!(mapper.sql_to_js_type("text"), "string");
        assert_eq!(mapper.sql_to_js_type("varchar"), "string");
        assert_eq!(mapper.sql_to_js_type("uuid"), "string");

        // Numeric types
        assert_eq!(mapper.sql_to_js_type("integer"), "number");
        assert_eq!(mapper.sql_to_js_type("float8"), "number");
        assert_eq!(mapper.sql_to_js_type("serial"), "number");

        // Boolean types
        assert_eq!(mapper.sql_to_js_type("boolean"), "boolean");
        assert_eq!(mapper.sql_to_js_type("bool"), "boolean");

        // Date/Time types
        assert_eq!(mapper.sql_to_js_type("timestamp"), "Date");
        assert_eq!(mapper.sql_to_js_type("timestamp with time zone"), "Date");

        // JSON types
        assert_eq!(mapper.sql_to_js_type("json"), "object");
        assert_eq!(mapper.sql_to_js_type("jsonb"), "object");

        // Array types
        assert_eq!(mapper.sql_to_js_type("text[]"), "array");
        assert_eq!(mapper.sql_to_js_type("integer[]"), "array");

        // Unknown type
        assert_eq!(mapper.sql_to_js_type("unknown_type_xxx"), "any");
    }

    #[test]
    fn test_cast_to_text_expr() {
        let mapper = PostgresMapper;
        assert_eq!(
            mapper.cast_to_text_expr("user_id", "integer"),
            "\"user_id\"::text AS \"user_id\""
        );
        assert_eq!(
            mapper.cast_to_text_expr("created_at", "timestamp"),
            "\"created_at\"::text AS \"created_at\""
        );
    }
}
