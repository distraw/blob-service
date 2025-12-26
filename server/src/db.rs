use sqlx::{PgPool, Row, migrate::Migrator, postgres::PgPoolOptions};
use eyre::{Result, eyre};
use std::path::Path;

const MIGRATION_FOLDER: &str = "./migrations";

#[derive(Clone, Debug, Default)]
pub struct Database {
    pool: Option<PgPool>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            pool: None,
        }
    }

    pub async fn connect(&mut self, url: String) -> Result<()> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await?;

        self.pool = Some(pool);

        Ok(())
    }

    pub async fn migrate(&mut self) -> Result<()> {
        let pool = self.pool
            .as_ref()
            .ok_or_else(||eyre!("connection with DB is not established"))?;

        Migrator::new(Path::new(MIGRATION_FOLDER)).await?.run(pool).await?;
        Ok(())
    }

    pub async fn insert(&self, content: String) -> Result<i32> {
        let pool = self.pool
            .as_ref()
            .ok_or_else(||eyre!("connection with DB is not established"))?;
        
        let row= sqlx::query(
                r#"INSERT INTO notes (content)
                VALUES ($1)
                RETURNING id"#)
            .bind(content)
            .fetch_one(pool)
            .await?;

        let id = row.try_get("id")?;
        Ok(id)
    }

    pub async fn fetch(&self, id: i32) -> Result<String> {
        let pool = self.pool
            .as_ref()
            .ok_or_else(||eyre!("connection with DB is not established"))?;

        let content: String = sqlx::query(
            r#"
            SELECT content
            FROM notes
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?
        .try_get(0)?;

        Ok(content)
    }
}