use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

pub struct ConnPool {
    conn_name: String,
    pool: sqlx::Pool<Postgres>,
}

// connect_to_db connects to postgres db
pub async fn connect_to_db(dsn: &str, conn_name: &str) -> Result<(ConnPool), sqlx::Error> {
    let pool = PgPoolOptions::new().max_connections(5).connect(dsn).await?;

    Ok(ConnPool {
        conn_name: conn_name.to_string(),
        pool,
    })
}
