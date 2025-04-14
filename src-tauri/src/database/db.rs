use crate::constants;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::types::chrono::Utc;
use sqlx::{query_as, Column, FromRow, Postgres, Row, ValueRef};

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

// connect_to_db connects to postgres db
pub async fn connect_to_db(
    username: &str,
    password: &str,
    hostname: &str,
    port: &i32,
    dbname: &str,
    conn_name: &str,
) -> Result<ConnPool, sqlx::Error> {
    let dsn = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, hostname, port, dbname,
    );
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
                dsn: dsn,
                pool,
            })
        }
        Err(e) => Err(e),
    }
}

impl ConnPool {
    pub async fn fetch_tables(&self) -> Result<Vec<TableSchema>, sqlx::Error> {
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
                        table_schema = 'public'
                        AND table_catalog = '{}'
            ",
            self.db_name,
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = query_as::<sqlx::Postgres, TableSchema>(&query)
            .fetch_all(&self.pool)
            .await;

        match query_result {
            Ok(row) => {
                return Ok(row);
            }
            Err(e) => {
                println!("{:#?}", e);
                return Err(e);
            }
        }
    }

    pub async fn fetch_table_columns(
        &self,
        table_name: &str,
    ) -> Result<Vec<TableColumns>, sqlx::Error> {
        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            "
                SELECT COLUMN_NAME,
                       DATA_TYPE
                FROM INFORMATION_SCHEMA.COLUMNS
                WHERE TABLE_CATALOG = '{}'
                  AND TABLE_SCHEMA = '{}'
                  AND TABLE_NAME = '{}';
            ",
            self.db_name, "public", table_name,
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = query_as::<sqlx::Postgres, TableColumns>(&query)
            .fetch_all(&self.pool)
            .await;

        match query_result {
            Ok(row) => {
                return Ok(row);
            }
            Err(e) => {
                println!("{:#?}", e);
                return Err(e);
            }
        }
    }

    pub async fn fetch_table_rows_count(
        &self,
        table_name: &str,
    ) -> Result<TableRowCount, sqlx::Error> {
        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT reltuples::bigint AS row_count
                FROM pg_class
                WHERE oid = 'public."{}"'::regclass;
            "#,
            table_name,
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = query_as::<sqlx::Postgres, TableRowCount>(&query)
            .fetch_one(&self.pool)
            .await;

        match query_result {
            Ok(row) => {
                return Ok(row);
            }
            Err(e) => {
                println!("{:#?}", e);
                return Err(e);
            }
        }
    }

    pub async fn fetch_table_data(
        &self,
        table_name: &str,
    ) -> Result<Vec<Vec<String>>, sqlx::Error> {
        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT *
                FROM {}
                LIMIT {};
            "#,
            table_name,
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
                return Ok(result);
            }
            Err(e) => {
                println!("{:#?}", e);
                return Err(e);
            }
        }
    }

    pub async fn fetch_table_data_with_offset(
        &self,
        table_name: &str,
        offset: &u32,
    ) -> Result<Vec<Vec<String>>, sqlx::Error> {
        // Removed: let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT *
                FROM {}
                LIMIT {} OFFSET {};
            "#,
            table_name,
            constants::INITIAL_PAGE_SIZE,
            offset
        );

        println!("Printing Query: {}", &query);

        // Execute directly on the pool
        let query_result = sqlx::query(&query).fetch_all(&self.pool).await;

        match query_result {
            Ok(row) => {
                let result = format_table_data(&row)?;
                return Ok(result);
            }
            Err(e) => {
                println!("{:#?}", e);
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
}

fn format_table_data(row: &Vec<PgRow>) -> Result<Vec<Vec<String>>, sqlx::Error> {
    let mut result: Vec<Vec<String>> = Vec::new();

    // looping through each row
    for (rowIndex, r) in row.iter().enumerate() {
        let mut row_result: Vec<String> = Vec::new();
        // looping through each column
        for (colIndex, col) in r.columns().iter().enumerate() {
            let colType = col.type_info().to_string();

            match colType.as_str() {
                constants::BOOL => {
                    let value: Option<bool> = r.get(colIndex);
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
                    let value: Option<i16> = r.get(colIndex);
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
                    let value: Option<i32> = r.get(colIndex);
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
                    let value: Option<i64> = r.get(colIndex);
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
                    let value: Option<f32> = r.get(colIndex);
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
                    let value: Option<f64> = r.get(colIndex);
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
                    let value: Option<String> = r.get(colIndex);
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
                    let value: Option<sqlx::types::chrono::DateTime<Utc>> = r.get(colIndex);
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
                    let value: Option<sqlx::types::chrono::NaiveDateTime> = r.get(colIndex);
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
                    let value: Option<serde_json::Value> = r.get(colIndex);
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
                    let value: Option<sqlx::types::uuid::Uuid> = r.get(colIndex);
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

    return Ok(result);
}
