use crate::error::FromDieselError;
use anyhow::{Context, anyhow};
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use r2d2::Pool;
use std::time::Duration;
use tracing::info;

pub mod dao;
pub mod database;
pub mod models;
pub mod schema;
pub mod converter;
pub mod error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../resources/migrations");

#[derive(Debug, Clone)]
pub struct Postgres {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(db_url: &str, username: &str, password: &str) -> Result<Postgres, r2d2::Error> {
        let url = format!("postgres://{}:{}@{}", username, password, db_url);
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder()
            .max_size(5)
            .connection_timeout(Duration::from_secs(5))
            .max_lifetime(None)
            .build(manager)?;
        Ok(Postgres { pool })
    }

    pub fn setup(&self) -> Result<(), anyhow::Error> {
        info!("setting up database");
        let mut connection = self.pool.get()?;
        connection.run_pending_migrations(MIGRATIONS).map_err(|e| anyhow!(e).context("Failed to run database migrations"))?;
        #[cfg(feature = "mock-user")]
        {
            use diesel::connection::SimpleConnection;
            connection.batch_execute("INSERT INTO user_account (user_name, display_name) VALUES ('anonymous', 'Anonymous') ON CONFLICT DO NOTHING;")?;
        }
        info!("database up to date");
        Ok(())
    }

    async fn run<F, T, E: FromDieselError + From<anyhow::Error>>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut PgConnection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().context("could not get database connection")?;
            f(&mut conn)
        })
        .await
        .context("database task panicked")?
        .map_err(E::from_diesel_error)
    }

    async fn transaction<F, T, E: FromDieselError + From<anyhow::Error>>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut PgConnection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.run(|conn| conn.transaction(|conn| f(conn))).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::connection::SimpleConnection;
    use diesel::r2d2::R2D2Connection;
    use r2d2::PooledConnection;

    impl Postgres {
        pub fn get_connection(
            &self,
        ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, anyhow::Error> {
            self.pool.get().context("could not get database connection")
        }
    }

    pub fn test_db() -> Postgres {
        let db = Postgres::new("localhost:55432/kanidb", "postgres" , "postgres").unwrap();
        let mut connection = db.get_connection().unwrap();
        connection
            .batch_execute(
                "DROP SCHEMA public CASCADE;\
                CREATE SCHEMA public;",
            )
            .unwrap();
        db.setup().unwrap();
        db
    }

    #[test]
    #[serial_test::serial]
    fn test_connect_db() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        conn.ping().unwrap();
    }
}
