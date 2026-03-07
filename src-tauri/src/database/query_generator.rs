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
