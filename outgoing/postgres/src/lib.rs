use anyhow::Context;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::{Pool, PooledConnection};
use std::error::Error;
use std::time::Duration;
use tracing::info;

pub mod dao;
pub mod database;
pub mod models;
pub mod schema;

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

    pub fn setup(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("setting up database");
        let mut connection = self.pool.get()?;
        connection.run_pending_migrations(MIGRATIONS)?;
        info!("database up to date");
        Ok(())
    }

    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, anyhow::Error> {
        self.pool.get().context("could not get database connection")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::connection::SimpleConnection;
    use diesel::r2d2::R2D2Connection;

    pub fn test_db() -> Postgres {
        let db = Postgres::new("localhost:55432", "postgres" , "postgres").unwrap();
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
