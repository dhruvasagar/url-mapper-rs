use anyhow::Error;
use hyper::Body;
use routerify::Router;

mod handlers;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/", handlers::index)
        .get("/new", handlers::new)
        .get("/:key/edit", handlers::edit)
        .build()
        .unwrap()
}
