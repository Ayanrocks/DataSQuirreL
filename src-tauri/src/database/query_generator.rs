use crate::types::api_objects::TransactionChange;
use sqlformat::{FormatOptions, QueryParams, format};
use std::collections::HashMap;

/// Escapes string value for SQL by doubling single quotes
fn escape_sql_string(val: &str) -> String {
    val.replace("'", "''")
}

/// Helper to format a value based on its presumed SQL type
fn format_sql_value(
    val: &str,
    col_name: &str,
    column_types: &Option<HashMap<String, String>>,
) -> String {
    let raw_type = column_types
        .as_ref()
        .and_then(|t| t.get(col_name))
        .cloned()
        .unwrap_or_else(|| "text".to_string());
    let type_lower = raw_type.to_lowercase();

    // Simple heuristic to determine if it should be quoted based on type
    let quote = match type_lower.as_str() {
        "integer" | "int" | "int4" | "int8" | "smallint" | "int2" | "bigint" | "decimal"
        | "numeric" | "real" | "float4" | "double precision" | "float8" | "serial"
        | "bigserial" | "smallserial" | "serial2" | "serial4" | "serial8" | "boolean" | "bool" => {
            false
        } // Numeric and boolean types don't need quotes
        _ => true, // String, Date, JSON, UUID, etc.
    };

    if quote {
        format!("'{}'::{}", escape_sql_string(val), raw_type)
    } else {
        // numeric/bool
        // handle empty strings for numerics as NULL if they somehow get here, though usually UI handles this
        if val.trim().is_empty() {
            "NULL".to_string()
        } else {
            // we can still cast numeric defaults if we want, but bare value is often fine.
            format!("{val}::{raw_type}")
        }
    }
}

pub fn generate_preview_queries(
    schema_name: &str,
    table_name: &str,
    changes: Vec<TransactionChange>,
    column_types: Option<HashMap<String, String>>,
) -> Result<Vec<String>, String> {
    let mut queries = Vec::new();
    let format_options = FormatOptions::default();

    for change in changes {
        match change.r#type.as_str() {
            "INSERT" => {
                let new_vals = change.new_values.unwrap_or_default();
                if new_vals.is_empty() {
                    continue;
                }

                let mut is_all_empty = true;
                for v in new_vals.values() {
                    if !v.trim().is_empty() {
                        is_all_empty = false;
                        break;
                    }
                }
                if is_all_empty {
                    return Err(
                        "Safety Guard: Attempted to insert a completely empty row".to_string()
                    );
                }

                let mut cols = Vec::new();
                let mut vals = Vec::new();

                for (k, v) in &new_vals {
                    cols.push(format!("\"{k}\""));
                    vals.push(format_sql_value(v, k, &column_types));
                }

                let query_str = format!(
                    "INSERT INTO \"{}\".\"{}\"\n  ({})\nVALUES\n  ({});",
                    schema_name,
                    table_name,
                    cols.join(", "),
                    vals.join(", ")
                );

                queries.push(format(&query_str, &QueryParams::None, &format_options));
            }
            "UPDATE" => {
                let new_vals = change.new_values.unwrap_or_default();
                let pk_vals = change.primary_keys.unwrap_or_default();
                let orig_vals = change.original_row.unwrap_or_default();

                if new_vals.is_empty() {
                    continue;
                }

                if pk_vals.is_empty() && orig_vals.is_empty() {
                    return Err(
                        "Safety Guard: Attempted UPDATE without WHERE clause identifiers"
                            .to_string(),
                    );
                }

                let mut set_clauses = Vec::new();
                let mut where_clauses = Vec::new();

                for (k, v) in &new_vals {
                    if let Some(orig_v) = orig_vals.get(k) {
                        if orig_v == v {
                            continue;
                        }
                    }

                    set_clauses.push(format!(
                        "\"{}\" = {}",
                        k,
                        format_sql_value(v, k, &column_types)
                    ));
                }

                if set_clauses.is_empty() {
                    continue;
                }

                if !pk_vals.is_empty() {
                    for (k, v) in &pk_vals {
                        where_clauses.push(format!(
                            "\"{}\" = {}",
                            k,
                            format_sql_value(v, k, &column_types)
                        ));
                    }
                } else {
                    for (k, v) in &orig_vals {
                        where_clauses.push(format!(
                            "\"{}\" = {}",
                            k,
                            format_sql_value(v, k, &column_types)
                        ));
                    }
                }

                let query_str = format!(
                    "UPDATE \"{}\".\"{}\"\nSET {}\nWHERE {};",
                    schema_name,
                    table_name,
                    set_clauses.join(", "),
                    where_clauses.join(" AND ")
                );

                queries.push(format(&query_str, &QueryParams::None, &format_options));
            }
            "DELETE" => {
                let pk_vals = change.primary_keys.unwrap_or_default();
                let orig_vals = change.original_row.unwrap_or_default();

                if pk_vals.is_empty() && orig_vals.is_empty() {
                    return Err(
                        "Safety Guard: Attempted DELETE without WHERE clause identifiers"
                            .to_string(),
                    );
                }

                let mut where_clauses = Vec::new();

                if !pk_vals.is_empty() {
                    for (k, v) in &pk_vals {
                        where_clauses.push(format!(
                            "\"{}\" = {}",
                            k,
                            format_sql_value(v, k, &column_types)
                        ));
                    }
                } else {
                    for (k, v) in &orig_vals {
                        where_clauses.push(format!(
                            "\"{}\" = {}",
                            k,
                            format_sql_value(v, k, &column_types)
                        ));
                    }
                }

                let query_str = format!(
                    "DELETE FROM \"{}\".\"{}\"\nWHERE {};",
                    schema_name,
                    table_name,
                    where_clauses.join(" AND ")
                );

                queries.push(format(&query_str, &QueryParams::None, &format_options));
            }
            _ => {}
        }
    }

    Ok(queries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_sql_value_string() {
        let types = Some(HashMap::from([("name".to_string(), "text".to_string())]));
        assert_eq!(
            format_sql_value("O'Connor", "name", &types),
            "'O''Connor'::text"
        );
    }

    #[test]
    fn test_format_sql_value_integer() {
        let types = Some(HashMap::from([("id".to_string(), "integer".to_string())]));
        assert_eq!(format_sql_value("42", "id", &types), "42::integer");
        assert_eq!(format_sql_value("  ", "id", &types), "NULL");
    }

    #[test]
    fn test_format_sql_value_unknown_type() {
        let types = None;
        assert_eq!(
            format_sql_value("O'Connor", "name", &types),
            "'O''Connor'::text"
        );
    }

    #[test]
    fn test_generate_insert_query() {
        let mut new_values = HashMap::new();
        new_values.insert("id".to_string(), "1".to_string());
        new_values.insert("name".to_string(), "Alice".to_string());

        let change = TransactionChange {
            r#type: "INSERT".to_string(),
            row_index: 0,
            primary_keys: None,
            original_row: None,
            new_values: Some(new_values),
        };

        let column_types = Some(HashMap::from([
            ("id".to_string(), "integer".to_string()),
            ("name".to_string(), "text".to_string()),
        ]));

        let queries =
            generate_preview_queries("public", "users", vec![change], column_types).unwrap();
        assert_eq!(queries.len(), 1);
        let q_clean = queries[0].replace(" ", "").replace("\n", "");
        assert!(q_clean.starts_with("INSERTINTO\"public\".\"users\""));
        assert!(q_clean.contains("\"id\""));
        assert!(q_clean.contains("\"name\""));
        assert!(q_clean.contains("1::integer"));
        assert!(q_clean.contains("'Alice'::text"));
    }

    #[test]
    fn test_generate_insert_empty_guard() {
        let mut new_values = HashMap::new();
        new_values.insert("name".to_string(), "  ".to_string());

        let change = TransactionChange {
            r#type: "INSERT".to_string(),
            row_index: 0,
            primary_keys: None,
            original_row: None,
            new_values: Some(new_values),
        };

        let err = generate_preview_queries("public", "users", vec![change], None).unwrap_err();
        assert_eq!(
            err,
            "Safety Guard: Attempted to insert a completely empty row"
        );
    }

    #[test]
    fn test_generate_update_query() {
        let mut new_values = HashMap::new();
        new_values.insert("name".to_string(), "Bob".to_string());

        let mut primary_keys = HashMap::new();
        primary_keys.insert("id".to_string(), "2".to_string());

        let change = TransactionChange {
            r#type: "UPDATE".to_string(),
            row_index: 0,
            primary_keys: Some(primary_keys),
            original_row: None,
            new_values: Some(new_values),
        };

        let column_types = Some(HashMap::from([
            ("id".to_string(), "integer".to_string()),
            ("name".to_string(), "text".to_string()),
        ]));

        let queries =
            generate_preview_queries("public", "users", vec![change], column_types).unwrap();
        assert_eq!(queries.len(), 1);
        let q_clean = queries[0].replace(" ", "").replace("\n", "");
        assert!(q_clean.starts_with("UPDATE\"public\".\"users\""));
        assert!(q_clean.contains("SET\"name\"='Bob'::text"));
        assert!(q_clean.contains("WHERE\"id\"=2::integer"));
    }

    #[test]
    fn test_generate_update_guard() {
        let mut new_values = HashMap::new();
        new_values.insert("name".to_string(), "Bob".to_string());

        let change = TransactionChange {
            r#type: "UPDATE".to_string(),
            row_index: 0,
            primary_keys: None,
            original_row: None,
            new_values: Some(new_values),
        };

        let err = generate_preview_queries("public", "users", vec![change], None).unwrap_err();
        assert_eq!(
            err,
            "Safety Guard: Attempted UPDATE without WHERE clause identifiers"
        );
    }

    #[test]
    fn test_generate_delete_query() {
        let mut original_row = HashMap::new();
        original_row.insert("id".to_string(), "3".to_string());
        original_row.insert("name".to_string(), "Charlie".to_string());

        let change = TransactionChange {
            r#type: "DELETE".to_string(),
            row_index: 0,
            primary_keys: None, // Testing fallback to original_row
            original_row: Some(original_row),
            new_values: None,
        };

        let column_types = Some(HashMap::from([
            ("id".to_string(), "integer".to_string()),
            ("name".to_string(), "text".to_string()),
        ]));

        let queries =
            generate_preview_queries("public", "users", vec![change], column_types).unwrap();
        assert_eq!(queries.len(), 1);
        let q_clean = queries[0].replace(" ", "").replace("\n", "");
        assert!(q_clean.starts_with("DELETEFROM\"public\".\"users\""));
        assert!(q_clean.contains("\"id\"=3::integer"));
        assert!(q_clean.contains("\"name\"='Charlie'::text"));
        assert!(q_clean.contains("AND"));
    }

    #[test]
    fn test_generate_delete_guard() {
        let change = TransactionChange {
            r#type: "DELETE".to_string(),
            row_index: 0,
            primary_keys: None,
            original_row: None,
            new_values: None,
        };

        let err = generate_preview_queries("public", "users", vec![change], None).unwrap_err();
        assert_eq!(
            err,
            "Safety Guard: Attempted DELETE without WHERE clause identifiers"
        );
    }
}
