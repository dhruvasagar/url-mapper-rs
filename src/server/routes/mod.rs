use crate::{db::Message, server::State};
use hyper::{
    Body,
    Request,
    Response,
};
use routerify::{
    Middleware,
    Router,
    RouterBuilder,
    ext::RequestExt,
    RequestInfo
};
use anyhow::{Error, Result};
use tracing::{info, error};

mod api;
mod admin;

async fn logger(req: Request<Body>) -> Result<Request<Body>> {
    info!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
    Ok(req)
}

async fn home_handler(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Url Mapper in Rust!")))
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    let status = match err.to_string().as_str() {
        "Unauthorized Access" => hyper::StatusCode::UNAUTHORIZED,
        _ => hyper::StatusCode::INTERNAL_SERVER_ERROR,
    };
    Response::builder()
        .status(status)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

async fn redirect_handler(req: Request<Body>) -> Result<Response<Body>> {
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
    let key = req.param("key").unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    sender_failed!(
        sender
        .send(Message::GetUrlMap { key: key.clone(), resp: tx})
        .await, "GetUrlMap");
    let url_map = recv_failed!(rx.await.unwrap());
    Ok(Response::builder()
       .header(hyper::header::LOCATION, url_map.url.clone())
       .status(hyper::StatusCode::SEE_OTHER)
       .body(Body::from(format!("redirecting to url: {}", url_map.url)))
       .unwrap())
}

pub fn router() -> RouterBuilder<Body, Error> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .get("/", home_handler)
        .get("/:key", redirect_handler)
        .scope("/api", api::router())
        .scope("/admin", admin::router())
        .err_handler_with_info(error_handler)
}
