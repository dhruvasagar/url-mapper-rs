use anyhow::Error;
use hyper::Body;
use routerify::Router;

mod url_maps;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .scope("/url_maps", url_maps::router())
        .build()
        .unwrap()
}
