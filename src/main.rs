use anyhow::Result;
use crate::db::{DB, Manager};
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;
use server::Server;

mod config;
mod db;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber)?;

    let db = DB::new().await.unwrap();
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut manager = Manager::new(db, db_rx);
        manager.listen().await;
    });

    Server::new(db_tx).listen().await?;

    Ok(())
}
