use hyper::{Body, Request, Response, StatusCode};
use routerify::{Error, Middleware, Router, ext::RequestExt, RequestInfo};
use tracing::{info, error};

async fn logger(req: Request<Body>) -> Result<Request<Body>, Error> {
    info!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
    Ok(req)
}

async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Url Mapper in Rust!")))
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .get("/", home_handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}
