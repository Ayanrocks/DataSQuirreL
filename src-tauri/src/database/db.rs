use crate::types::db::{ConnPool, TableColumns, TableRowCount, TableSchema};
use crate::{constants, log_function};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::types::chrono::Utc;
use sqlx::{Column, Row, query_as};

// connect_to_db connects to postgres db
pub async fn connect_to_db(
    username: &str,
    password: &str,
    hostname: &str,
    port: &i32,
    dbname: &str,
    conn_name: &str,
) -> Result<ConnPool, sqlx::Error> {
    log_function!(connect_to_db);
    let dsn = format!("postgres://{username}:{password}@{hostname}:{port}/{dbname}",);
    let pool_result = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5)) // wait 5 seconds
        .connect(dsn.as_str())
        .await;

    match pool_result {
        Ok(pool) => {
            // run vacuum analyze to update the db statistics
            sqlx::query(r#"VACUUM ANALYZE"#).execute(&pool).await?;

            Ok(ConnPool {
                conn_name: conn_name.to_string(),
                db_name: dbname.to_string(),
                dsn,
                pool,
            })
        }
        Err(e) => Err(e),
    }
}

impl ConnPool {
    pub async fn fetch_schemas(&self) -> Result<Vec<String>, sqlx::Error> {
        log_function!(fetch_schemas);
        let query = format!(
            "
                    SELECT schema_name
                    FROM information_schema.schemata
                    WHERE catalog_name = '{}'
                    AND schema_name NOT IN ('pg_catalog', 'information_schema');
            ",
            self.db_name,
        );

        println!("Printing Query: {}", &query);

        let query_result = sqlx::query(&query).fetch_all(&self.pool).await;

        match query_result {
            Ok(rows) => {
                let schemas = rows.into_iter().map(|row| row.get("schema_name")).collect();
                Ok(schemas)
            }
            Err(e) => {
                println!("{e:#?}");
                Err(e)
            }
        }
    }

    pub async fn fetch_tables(&self, schema_name: &str) -> Result<Vec<TableSchema>, sqlx::Error> {
        log_function!(fetch_tables);
        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            "
                    SELECT
                        table_catalog,
                        table_schema,
                        table_name,
                        table_type
                    FROM
                        information_schema.tables
                    WHERE
                        table_schema = '{}'
                        AND table_catalog = '{}'
            ",
            schema_name, self.db_name,
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = query_as::<sqlx::Postgres, TableSchema>(&query)
            .fetch_all(&self.pool)
            .await;

        match query_result {
            Ok(row) => Ok(row),
            Err(e) => {
                println!("{e:#?}");
                Err(e)
            }
        }
    }

    pub async fn fetch_table_columns(
        &self,
        database_name: &str,
        schema_name: &str,
        table_name: &str,
    ) -> Result<Vec<TableColumns>, sqlx::Error> {
        log_function!(fetch_table_columns);
        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            "
                SELECT COLUMN_NAME,
                       DATA_TYPE
                FROM INFORMATION_SCHEMA.COLUMNS
                WHERE TABLE_CATALOG = '{database_name}'
                  AND TABLE_SCHEMA = '{schema_name}'
                  AND TABLE_NAME = '{table_name}';
            ",
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = query_as::<sqlx::Postgres, TableColumns>(&query)
            .fetch_all(&self.pool)
            .await;

        match query_result {
            Ok(row) => Ok(row),
            Err(e) => {
                println!("{e:#?}");
                Err(e)
            }
        }
    }

    pub async fn fetch_table_primary_keys(
        &self,
        schema_name: &str,
        table_name: &str,
    ) -> Result<Vec<String>, sqlx::Error> {
        log_function!(fetch_table_primary_keys);

        let query = format!(
            "
            SELECT kcu.column_name
            FROM information_schema.table_constraints tco
            JOIN information_schema.key_column_usage kcu 
              ON kcu.constraint_name = tco.constraint_name 
              AND kcu.constraint_schema = tco.constraint_schema 
              AND kcu.constraint_name = tco.constraint_name 
            WHERE tco.constraint_type = 'PRIMARY KEY'
              AND kcu.table_schema = '{schema_name}'
              AND kcu.table_name = '{table_name}';
            "
        );

        let query_result = sqlx::query(&query).fetch_all(&self.pool).await;

        match query_result {
            Ok(rows) => {
                let pks: Vec<String> = rows.into_iter().map(|row| row.get("column_name")).collect();
                Ok(pks)
            }
            Err(e) => {
                println!("[fetch_table_primary_keys] Error: {e:#?}");
                Err(e)
            }
        }
    }

    pub async fn fetch_table_foreign_keys(
        &self,
        schema_name: &str,
        table_name: &str,
    ) -> Result<Vec<String>, sqlx::Error> {
        log_function!(fetch_table_foreign_keys);

        let query = format!(
            "
            SELECT kcu.column_name
            FROM information_schema.table_constraints tco
            JOIN information_schema.key_column_usage kcu 
              ON kcu.constraint_name = tco.constraint_name 
              AND kcu.constraint_schema = tco.constraint_schema 
              AND kcu.constraint_name = tco.constraint_name 
            WHERE tco.constraint_type = 'FOREIGN KEY'
              AND kcu.table_schema = '{schema_name}'
              AND kcu.table_name = '{table_name}';
            "
        );

        let query_result = sqlx::query(&query).fetch_all(&self.pool).await;

        match query_result {
            Ok(rows) => {
                let fks: Vec<String> = rows.into_iter().map(|row| row.get("column_name")).collect();
                Ok(fks)
            }
            Err(e) => {
                println!("[fetch_table_foreign_keys] Error: {e:#?}");
                Err(e)
            }
        }
    }

    pub async fn fetch_table_rows_count(
        &self,
        schema_name: &str,
        table_name: &str,
    ) -> Result<TableRowCount, sqlx::Error> {
        log_function!(fetch_table_rows_count);
        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT reltuples::bigint AS row_count
                FROM pg_class
                WHERE oid = '"{schema_name}"."{table_name}"'::regclass;
            "#,
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = query_as::<sqlx::Postgres, TableRowCount>(&query)
            .fetch_one(&self.pool)
            .await;

        match query_result {
            Ok(row) => Ok(row),
            Err(e) => {
                println!("{e:#?}");
                Err(e)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn fetch_table_data(
        &self,
        schema_name: &str,
        table_name: &str,
        columns: &[TableColumns],
        mapper: &dyn crate::database::types_mapper::DbTypeMapper,
        sort_column: &Option<String>,
        sort_direction: &Option<String>,
        where_clause: &Option<String>,
    ) -> Result<Vec<Vec<String>>, sqlx::Error> {
        log_function!(fetch_table_data);

        let mut select_cols = Vec::new();
        for col in columns {
            select_cols.push(mapper.cast_to_text_expr(&col.column_name, &col.data_type));
        }
        let cols_str = if select_cols.is_empty() {
            "*".to_string()
        } else {
            select_cols.join(", ")
        };

        let mut order_by_str = String::new();
        if let Some(col) = sort_column {
            if let Some(dir) = sort_direction {
                if dir.to_lowercase() == "asc" || dir.to_lowercase() == "desc" {
                    order_by_str = format!(
                        "ORDER BY \"{}\".\"{}\".\"{}\" {}",
                        schema_name,
                        table_name,
                        col,
                        dir.to_uppercase()
                    );
                }
            }
        }

        let mut where_str = String::new();
        if let Some(clause) = where_clause {
            if !clause.trim().is_empty() {
                where_str = format!("WHERE {clause}");
            }
        }

        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT {}
                FROM "{}"."{}"
                {}
                {}
                LIMIT {};
            "#,
            cols_str,
            schema_name,
            table_name,
            where_str,
            order_by_str,
            constants::INITIAL_PAGE_SIZE
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = sqlx::query::<sqlx::Postgres>(&query)
            .fetch_all(&self.pool)
            .await;

        match query_result {
            Ok(row) => {
                let result = format_table_data(&row)?;
                Ok(result)
            }
            Err(e) => {
                println!("{e:#?}");
                Err(e)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn fetch_table_data_with_offset(
        &self,
        schema_name: &str,
        table_name: &str,
        offset: &u32,
        limit: &Option<u32>,
        columns: &[TableColumns],
        mapper: &dyn crate::database::types_mapper::DbTypeMapper,
        sort_column: &Option<String>,
        sort_direction: &Option<String>,
        where_clause: &Option<String>,
    ) -> Result<Vec<Vec<String>>, sqlx::Error> {
        log_function!(fetch_table_data_with_offset);
        // Removed: let mut db_conn = self.pool.acquire().await?;

        let mut select_cols = Vec::new();
        for col in columns {
            select_cols.push(mapper.cast_to_text_expr(&col.column_name, &col.data_type));
        }
        let cols_str = if select_cols.is_empty() {
            "*".to_string()
        } else {
            select_cols.join(", ")
        };

        let limit_str = match limit {
            Some(l) => l.to_string(),
            None => "ALL".to_string(),
        };

        let mut order_by_str = String::new();
        if let Some(col) = sort_column {
            if let Some(dir) = sort_direction {
                if dir.to_lowercase() == "asc" || dir.to_lowercase() == "desc" {
                    order_by_str = format!(
                        "ORDER BY \"{}\".\"{}\".\"{}\" {}",
                        schema_name,
                        table_name,
                        col,
                        dir.to_uppercase()
                    );
                }
            }
        }

        let mut where_str = String::new();
        if let Some(clause) = where_clause {
            if !clause.trim().is_empty() {
                where_str = format!("WHERE {clause}");
            }
        }

        let query = format!(
            r#"
                SELECT {cols_str}
                FROM "{schema_name}"."{table_name}"
                {where_str}
                {order_by_str}
                LIMIT {limit_str} OFFSET {offset};
            "#
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = sqlx::query(&query).fetch_all(&self.pool).await;

        match query_result {
            Ok(row) => {
                let result = format_table_data(&row)?;
                Ok(result)
            }
            Err(e) => {
                println!("{e:#?}");
                // return Err(e);
                Err(e)
            }
        }
    }

    // pub async fn update_table_data(
    //     &self,
    //     table_name: &str,
    //     where_clause: &HashMap<String, String>,
    //     update_map: &HashMap<String, String>,
    // ) -> Result<TableRowCount, sqlx::Error> {
    //     let mut db_conn = self.pool.acquire().await?;
    //     let mut where_string: String = "".to_string();
    //     let mut update_string: String = "".to_string();

    //     for (key, val) in where_clause.into_iter() {
    //         where_string = format!("{} AND {} = {}", where_string, key, val)
    //     }

    //     for (key, val) in update_map.into_iter() {
    //         update_string = format!("{}, {} = {}", update_string, key, val)
    //     }

    //     // looping through the where map to form a where string.

    //     let query_string = format!(
    //         r#"
    //             UPDATE {} SET {} WHERE {}
    //         "#,
    //         table_name, update_string, where_string
    //     );

    //     // println!("Printing Query: {}", &query);

    //     let query_result = sqlx::query::Query(&query_string)
    //         .execute(&mut db_conn)
    //         .await;

    //     match query_result {
    //         Ok(row) => {
    //             return Ok(row);
    //         }
    //         Err(e) => {
    //             println!("{:#?}", e);
    //             return Err(e);
    //         }
    //     }
    // }

    // pub async fn raw_query_runner(
    //     &self,
    //     table_name: &str,
    //     query_string: &str,
    // ) -> Result<Vec<Vec<String>>, sqlx::Error> {
    //     let mut db_conn = self.pool.acquire().await?;

    //     // looping through the where map to form a where string.

    //     let query = query_string;

    //     println!("Printing Query: {}", &query);

    //     let query_result = query_as::<sqlx::Postgres, Any>(&query)
    //         .fetch_one(&mut db_conn)
    //         .await;

    //     match query_result {
    //         Ok(row) => {
    //             return Ok(row);
    //         }
    //         Err(e) => {
    //             println!("{:#?}", e);
    //             return Err(e);
    //         }
    //     }
    // }

    pub async fn commit_transaction(
        &self,
        schema_name: &str,
        table_name: &str,
        changes: Vec<crate::types::api_objects::TransactionChange>,
        column_types: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Optional: SET TRANSACTION ISOLATION LEVEL READ COMMITTED;
        // In PostgreSQL, READ COMMITTED is the default.

        for change in changes {
            match change.r#type.as_str() {
                "INSERT" => {
                    let new_vals = change.new_values.unwrap_or_default();
                    if new_vals.is_empty() {
                        continue;
                    }

                    // GUARD: Never commit a completely empty insert row
                    let mut is_all_empty = true;
                    for v in new_vals.values() {
                        if !v.trim().is_empty() {
                            is_all_empty = false;
                            break;
                        }
                    }
                    if is_all_empty {
                        return Err(sqlx::Error::Protocol(
                            "Safety Guard: Attempted to insert a completely empty row".to_string(),
                        ));
                    }

                    let mut cols = Vec::new();
                    let mut vals = Vec::new();
                    let mut placeholders = Vec::new();

                    let mut idx = 1;
                    for (k, v) in &new_vals {
                        let cast = match &column_types {
                            Some(types_map) => match types_map.get(k) {
                                Some(raw_type) => format!("::{raw_type}"),
                                None => "".to_string(),
                            },
                            None => "".to_string(),
                        };
                        cols.push(format!("\"{k}\""));
                        vals.push(v);
                        placeholders.push(format!("${idx}{cast}"));
                        idx += 1;
                    }

                    let query_str = format!(
                        "INSERT INTO \"{}\".\"{}\" ({}) VALUES ({})",
                        schema_name,
                        table_name,
                        cols.join(", "),
                        placeholders.join(", ")
                    );

                    let mut q = sqlx::query(&query_str);
                    for v in vals {
                        q = q.bind(v);
                    }

                    q.execute(&mut *tx).await?;
                }
                "UPDATE" => {
                    let new_vals = change.new_values.unwrap_or_default();
                    let pk_vals = change.primary_keys.unwrap_or_default();
                    let orig_vals = change.original_row.unwrap_or_default();

                    if new_vals.is_empty() {
                        continue;
                    }

                    // GUARD: Never update without a WHERE clause
                    if pk_vals.is_empty() && orig_vals.is_empty() {
                        return Err(sqlx::Error::Protocol(
                            "Safety Guard: Attempted UPDATE without WHERE clause identifiers"
                                .to_string(),
                        ));
                    }

                    let mut set_clauses = Vec::new();
                    let mut where_clauses = Vec::new();
                    let mut binds: Vec<&String> = Vec::new();

                    let mut idx = 1;

                    for (k, v) in &new_vals {
                        if let Some(orig_v) = orig_vals.get(k) {
                            if orig_v == v {
                                continue;
                            }
                        }

                        let cast = match &column_types {
                            Some(types_map) => match types_map.get(k) {
                                Some(raw_type) => format!("::{raw_type}"),
                                None => "".to_string(),
                            },
                            None => "".to_string(),
                        };
                        set_clauses.push(format!("\"{k}\" = ${idx}{cast}"));
                        binds.push(v);
                        idx += 1;
                    }

                    if set_clauses.is_empty() {
                        continue;
                    }

                    // Build WHERE clause
                    if !pk_vals.is_empty() {
                        for (k, v) in &pk_vals {
                            let cast = match &column_types {
                                Some(types_map) => match types_map.get(k) {
                                    Some(raw_type) => format!("::{raw_type}"),
                                    None => "".to_string(),
                                },
                                None => "".to_string(),
                            };
                            where_clauses.push(format!("\"{k}\" = ${idx}{cast}"));
                            binds.push(v);
                            idx += 1;
                        }
                    } else {
                        // fallback to all original columns
                        for (k, v) in &orig_vals {
                            let cast = match &column_types {
                                Some(types_map) => match types_map.get(k) {
                                    Some(raw_type) => format!("::{raw_type}"),
                                    None => "".to_string(),
                                },
                                None => "".to_string(),
                            };
                            where_clauses.push(format!("\"{k}\" = ${idx}{cast}"));
                            binds.push(v);
                            idx += 1;
                        }
                    }

                    let query_str = format!(
                        "UPDATE \"{}\".\"{}\" SET {} WHERE {}",
                        schema_name,
                        table_name,
                        set_clauses.join(", "),
                        where_clauses.join(" AND ")
                    );

                    let mut q = sqlx::query(&query_str);
                    for b in binds {
                        q = q.bind(b);
                    }

                    q.execute(&mut *tx).await?;
                }
                "DELETE" => {
                    let pk_vals = change.primary_keys.unwrap_or_default();
                    let orig_vals = change.original_row.unwrap_or_default();

                    // GUARD: Never delete without a WHERE clause
                    if pk_vals.is_empty() && orig_vals.is_empty() {
                        return Err(sqlx::Error::Protocol(
                            "Safety Guard: Attempted DELETE without WHERE clause identifiers"
                                .to_string(),
                        ));
                    }

                    let mut where_clauses = Vec::new();
                    let mut binds: Vec<&String> = Vec::new();
                    let mut idx = 1;

                    if !pk_vals.is_empty() {
                        for (k, v) in &pk_vals {
                            let cast = match &column_types {
                                Some(types_map) => match types_map.get(k) {
                                    Some(raw_type) => format!("::{raw_type}"),
                                    None => "".to_string(),
                                },
                                None => "".to_string(),
                            };
                            where_clauses.push(format!("\"{k}\" = ${idx}{cast}"));
                            binds.push(v);
                            idx += 1;
                        }
                    } else {
                        for (k, v) in &orig_vals {
                            let cast = match &column_types {
                                Some(types_map) => match types_map.get(k) {
                                    Some(raw_type) => format!("::{raw_type}"),
                                    None => "".to_string(),
                                },
                                None => "".to_string(),
                            };
                            where_clauses.push(format!("\"{k}\" = ${idx}{cast}"));
                            binds.push(v);
                            idx += 1;
                        }
                    }

                    let query_str = format!(
                        "DELETE FROM \"{}\".\"{}\" WHERE {}",
                        schema_name,
                        table_name,
                        where_clauses.join(" AND ")
                    );

                    let mut q = sqlx::query(&query_str);
                    for b in binds {
                        q = q.bind(b);
                    }

                    q.execute(&mut *tx).await?;
                }
                _ => {}
            }
        }

        tx.commit().await?;
        Ok(())
    }
}

fn format_table_data(row: &[PgRow]) -> Result<Vec<Vec<String>>, sqlx::Error> {
    log_function!(format_table_data);
    let mut result: Vec<Vec<String>> = Vec::new();

    // looping through each row
    for r in row.iter() {
        let mut row_result: Vec<String> = Vec::new();
        // looping through each column
        for (col_index, col) in r.columns().iter().enumerate() {
            let col_type = col.type_info().to_string();

            match col_type.as_str() {
                constants::BOOL => {
                    let value: Option<bool> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::SMALLINT | constants::SMALLSERIAL | constants::INT2 => {
                    let value: Option<i16> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::INT | constants::SERIAL | constants::INT4 => {
                    let value: Option<i32> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::BIGINT | constants::BIGSERIAL | constants::INT8 => {
                    let value: Option<i64> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::FLOAT4 | constants::REAL => {
                    let value: Option<f32> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::FLOAT8 | constants::DOUBLE_PRECISION => {
                    let value: Option<f64> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::VARCHAR
                | constants::CHAR
                | constants::TEXT
                | constants::CITEXT
                | constants::NAME => {
                    let value: Option<String> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val);
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::TIMESTAMPTZ => {
                    let value: Option<sqlx::types::chrono::DateTime<Utc>> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::TIMESTAMP => {
                    let value: Option<sqlx::types::chrono::NaiveDateTime> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    };
                }
                constants::JSON | constants::JSONB => {
                    let value: Option<serde_json::Value> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    }
                }
                constants::UUID => {
                    let value: Option<sqlx::types::uuid::Uuid> = r.get(col_index);
                    match value {
                        Some(val) => {
                            row_result.push(val.to_string());
                        }
                        None => {
                            row_result.push("NULL".to_string());
                        }
                    }
                }
                &_ => {
                    row_result.push("NULL".to_string());
                }
            }
        }
        result.push(row_result);
    }

    Ok(result)
}
