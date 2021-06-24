use hyper::Server as HyperServer;
use crate::{db::Message, config::CONFIG};
use routerify::RouterService;
use anyhow::Result;
use tracing::info;
use super::routes;
use tokio::sync::mpsc::Sender;

pub struct Server {
    db_sender: Sender<Message>,
}

impl Server {
    pub fn new(db_sender: Sender<Message>) -> Self {
        Self { db_sender }
    }

    pub async fn listen(&self) -> Result<()> {
        let router = routes::router()
            .data(self.db_sender.clone())
            .build()
            .unwrap();
        let service = RouterService::new(router).unwrap();
        let addr = format!("{}:{}", CONFIG.host, CONFIG.port)
            .parse()?;
        let server = HyperServer::bind(&addr).serve(service);
        info!("Server started listening on {}", addr);
        server.await?;
        Ok(())
    }
}
