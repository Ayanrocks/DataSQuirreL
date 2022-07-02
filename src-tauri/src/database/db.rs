use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{query_as, FromRow, Postgres};

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
        "posrgres://{}:{}@{}:{}/{}",
        username, password, hostname, port, dbname,
    );
    let pool_result = PgPoolOptions::new()
        .max_connections(5)
        .connect(dsn.as_str())
        .await;

    match pool_result {
        Ok(pool) => Ok(ConnPool {
            conn_name: conn_name.to_string(),
            db_name: dbname.to_string(),
            dsn: dsn,
            pool,
        }),
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
        print!("\nInside Function\n");
        match query_result {
            Ok(row) => {
                print!("\nInside Function\n");
                println!("{:?}", row);
                return Ok(row);
            }
            Err(e) => {
                println!("{:#?}", e);
                return Err(e);
            }
        }
    }
}
