use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{query, query_as, Column, FromRow, Postgres, Row, ValueRef};
use std::any::Any;
use std::borrow::Borrow;

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
    pub row_count: String,
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
        let mut db_conn = self.pool.acquire().await?;
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

        let query_result = query_as::<sqlx::Postgres, TableSchema>(&query)
            .fetch_all(&mut db_conn)
            .await;

        match query_result {
            Ok(row) => {
                println!("{:?}", row);
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
        let mut db_conn = self.pool.acquire().await?;
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

        let query_result = query_as::<sqlx::Postgres, TableColumns>(&query)
            .fetch_all(&mut db_conn)
            .await;

        match query_result {
            Ok(row) => {
                println!("{:?}", row);
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
        let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT reltuples::bigint AS row_count
                FROM pg_class
                WHERE oid = 'public."{}"'::regclass;
            "#,
            table_name,
        );

        println!("Printing Query: {}", &query);

        let query_result = query_as::<sqlx::Postgres, TableRowCount>(&query)
            .fetch_one(&mut db_conn)
            .await;

        match query_result {
            Ok(row) => {
                println!("{:?}", row);
                return Ok(row);
            }
            Err(e) => {
                println!("{:#?}", e);
                return Err(e);
            }
        }
    }

    pub async fn fetch_table_data(&self, table_name: &str) -> Result<(), sqlx::Error> {
        let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT *
                FROM {}
                LIMIT 600;
            "#,
            table_name,
        );

        println!("Printing Query: {}", &query);

        let query_result = sqlx::query::<sqlx::Postgres>(&query)
            .fetch_all(&mut db_conn)
            .await;

        match query_result {
            Ok(row) => {
                for v in row {
                    let _val: ty = v.try_get(6).unwrap();

                    // println!("Row Result: {:?}", *_val);
                    dbg!(_val.);
                }
                return Ok(());
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
    ) -> Result<(), sqlx::Error> {
        let mut db_conn = self.pool.acquire().await?;
        let query = format!(
            r#"
                SELECT *
                FROM {}
                LIMIT 600 OFFSET {};
            "#,
            table_name, offset
        );

        println!("Printing Query: {}", &query);

        let query_result = sqlx::query(&query).fetch_all(&mut db_conn).await;

        match query_result {
            Ok(row) => {
                for r in row {
                    println!("{:#?}", r.len());
                }
                Ok(())
            }
            Err(e) => {
                println!("{:#?}", e);
                // return Err(e);
                Err(e)
            }
        }
    }
}
