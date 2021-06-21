use hyper::Server;
use crate::config::CONFIG;
use routerify::RouterService;
use anyhow::Result;
use tracing::info;
use super::routes;

pub async fn listen() -> Result<()> {
    let router = routes::router();
    let service = RouterService::new(router).unwrap();
    let addr = format!("{}:{}", CONFIG.host, CONFIG.port)
        .parse()?;
    let server = Server::bind(&addr).serve(service);
    info!("Server started listening on {}", addr);
    server.await?;
    Ok(())
}
