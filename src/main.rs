use anyhow::Result;
use crate::config::CONFIG;
use crate::db::{DB, Message, Manager};

mod config;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "host: {}, port: {}, database.url: {}",
        CONFIG.host, CONFIG.port, CONFIG.database.url
    );

    let db = DB::new().await.unwrap();
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut manager = Manager::new(db, db_rx);
        manager.listen().await;
    });

    let (tx, rx) = tokio::sync::oneshot::channel();
    match db_tx.send(Message::DeleteUrlMap { key: "linkedin".into(), resp: tx }).await {
        Ok(_) => {},
        Err(e) => eprintln!("Failed to send to database manager: {}", e)
    }
    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Unable to get url_maps: {}", e)
    }

    Ok(())
}
