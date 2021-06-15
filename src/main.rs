use anyhow::Result;
use crate::config::CONFIG;
use crate::db::{DB, UrlMap};

mod config;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "host: {}, port: {}, database.url: {}",
        CONFIG.host, CONFIG.port, CONFIG.database.url
    );

    let db = DB::new().await.unwrap();
    let res = sqlx::query_as::<_, UrlMap>("select * from url_maps")
        .fetch_all(&db.pool)
        .await?;

    println!("results: {:?}", res);

    Ok(())
}
