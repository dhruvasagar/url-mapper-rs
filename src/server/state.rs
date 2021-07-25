use crate::{config::CONFIG, db::Message};
use anyhow::Result;
use tera::Tera;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct State {
    db_sender: Sender<Message>,
    tera: Tera,
}

impl State {
    pub fn new(db_sender: Sender<Message>) -> Result<Self> {
        let tera = Tera::new("client/tera/**/*.html")?;
        Ok(Self { db_sender, tera })
    }

    pub fn db_sender(&self) -> Sender<Message> {
        self.db_sender.clone()
    }

    pub fn tera(&self) -> Tera {
        let mut tera = self.tera.clone();
        if CONFIG.env.as_str() == "development" {
            match tera.full_reload() {
                Ok(_) => tracing::info!("Tera templates reloaded successfully!"),
                Err(e) => tracing::error!("Failed to reload tera templates: {}", e),
            }
        }
        tera
    }
}
