use crate::db::Message;
use hyper::{
    Body,
    Request,
    Response,
    StatusCode
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
use tokio::sync::mpsc::Sender;

mod api;

async fn logger(req: Request<Body>) -> Result<Request<Body>> {
    info!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
    Ok(req)
}

async fn home_handler(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Url Mapper in Rust!")))
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

macro_rules! sender_failed {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {},
            Err(e) => {
                error!("Database Manager failed to get {}! error: {}", $f, e);
                return Ok(Response::builder()
                          .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                          .body(Body::from(format!("Something went wrong: {}", e)))
                          .unwrap());
            }
        }
    }
}

macro_rules! recv_failed {
    ($m: expr) => {
        match $m {
            Ok(d) => d,
            Err(e) => {
                error!("Database Manager returned error: {}", e);
                return Ok(Response::builder()
                          .status(hyper::StatusCode::NOT_FOUND)
                          .body(Body::from("Key does not exist"))
                          .unwrap());
            }
        }
    }
}

async fn redirect_handler(req: Request<Body>) -> Result<Response<Body>> {
    let sender = req.data::<Sender<Message>>().unwrap();
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
        .err_handler_with_info(error_handler)
}
