use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use std::env;

pub struct ConnPool {
    pub conn_name: String,
    pub db_name: String,
    pub dsn: String,
    pub pool: sqlx::Pool<Postgres>,
}

#[derive(Debug)]
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
        Ok(pool) => {
            env::set_var("DATABASE_URL", &dsn);
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
    pub async fn fetch_tables(&self) {
        let db_conn_result = self.pool.acquire().await;
        match db_conn_result {
            Ok(mut db_conn) => async {
                let query_result = sqlx::query_as(
                    format!(
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
                        AND table_catalog = '{}'",
                        self.dsn,
                    )
                    .as_str(),
                )
                .fetch_all(&mut db_conn)
                .await;

                match query_result {
                    Ok(v) => {
                        println!("Vec: {}", v)
                    },
                    Err(_) => todo!(),
                };
            },
            Err(e) => {
                println!("Error Fetching: {}", e)
            }
        }
    }
}
