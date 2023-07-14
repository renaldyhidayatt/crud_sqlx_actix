use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type ConnectionPool = Pool<Postgres>;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn new_pool(
        connection_string: &str,
        run_migrations: bool,
    ) -> Result<ConnectionPool, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;

        if run_migrations {
            sqlx::migrate!().run(&pool).await?;
        }

        Ok(pool)
    }
}
