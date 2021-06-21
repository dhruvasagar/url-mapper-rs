use anyhow::Result;
use crate::config::CONFIG;
use crate::db::{DB, Manager};
use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;

mod config;
mod db;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber)?;

    info!(
        "host: {}, port: {}, database.url: {}",
        CONFIG.host, CONFIG.port, CONFIG.database.url
    );

    let db = DB::new().await.unwrap();
    let (_db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut manager = Manager::new(db, db_rx);
        manager.listen().await;
    });

    server::listen().await?;

    Ok(())
}
