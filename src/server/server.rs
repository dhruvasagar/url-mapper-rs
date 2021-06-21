use hyper::Server;
use crate::config::CONFIG;
use routerify::RouterService;
use anyhow::Result;
use crate::server::routes::router;
use tracing::info;

pub async fn listen() -> Result<()> {
    let router = router();
    let service = RouterService::new(router).unwrap();
    let addr = format!("{}:{}", CONFIG.host, CONFIG.port)
        .parse()?;
    let server = Server::bind(&addr).serve(service);
    info!("Server started listening on {}", addr);
    server.await?;
    Ok(())
}
