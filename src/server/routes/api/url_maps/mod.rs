use anyhow::Error;
use hyper::Body;
use routerify::Router;

mod handlers;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/", handlers::get_url_maps)
        .post("/", handlers::create_url_map)
        .get("/:key", handlers::get_url_map)
        .put("/:key", handlers::update_url_map)
        .delete("/:key", handlers::delete_url_map)
        .build()
        .unwrap()
}
