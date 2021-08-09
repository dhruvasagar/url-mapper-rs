use anyhow::{anyhow, Error, Result};
use hyper::{Body, Request};
use routerify::{Router, Middleware};
use base64::decode;
use std::str::from_utf8;
use crate::config::CONFIG;

mod url_maps;

fn validate_token(encoded_token: &str) -> Result<()> {
    let auth_token_bytes = decode(&encoded_token)?;
    let auth_token = from_utf8(&auth_token_bytes)?;
    if auth_token != CONFIG.auth_token.as_str() {
        return Err(anyhow!("Unauthorized Access"));
    }
    Ok(())
}

async fn auth_middleware(req: Request<Body>) -> Result<Request<Body>> {
    if req.method() == hyper::Method::OPTIONS {
        return Ok(req);
    }

    let auth_token_header = req.headers().get(hyper::header::AUTHORIZATION);
    match auth_token_header {
        None => Err(anyhow!("Unauthorized Access")),
        Some(auth_token) => {
            let token = auth_token.to_str()?;
            validate_token(token)?;
            Ok(req)
        }
    }
}

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .middleware(Middleware::pre(auth_middleware))
        .scope("/url_maps", url_maps::router())
        .build()
        .unwrap()
}
