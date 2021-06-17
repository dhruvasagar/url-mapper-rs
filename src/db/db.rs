use sqlx::{migrate, Pool, Postgres, postgres::PgPoolOptions, FromRow};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use crate::config::CONFIG;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UrlMap {
    pub key: String,
    pub url: String,
}

impl UrlMap {
    pub fn new(key: String, url: String) -> Self {
        Self { key, url }
    }
}

pub struct DB {
    pub pool: Pool<Postgres>,
}

impl DB {
    pub async fn new() -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(CONFIG.database.max_connections)
            .connect(&CONFIG.database.url)
            .await?;

        let migrator = migrate!();
        migrator
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }
}
