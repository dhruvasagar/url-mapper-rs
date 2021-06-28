use anyhow::Result;
use serde::{Serialize, Deserialize};
use hyper::{Body, Request, Response, body::to_bytes};
use routerify::ext::RequestExt;
use tokio::sync::mpsc::Sender;
use crate::db::{UrlMap, Message};

macro_rules! json_response {
    (body: $body:expr) => {
        Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string($body).unwrap().into())
            .unwrap()
    };
    (status: $status:expr, body: $body:expr) => {
        Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .status($status)
            .body(serde_json::to_string($body).unwrap().into())
            .unwrap()
    };
    (error: $e:expr) => {
        json_response!(
            status: hyper::StatusCode::INTERNAL_SERVER_ERROR,
            body: &serde_json::json!({
                "error": $e.to_string(),
            }).to_string())
    };
}

macro_rules! sender_failed_json {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Database Manager failed to get {}! error: {}", $f, e);
                return Ok(json_response!(error: e));
            }
        }
    }
}

macro_rules! recv_failed_json {
    ($m: expr, $status: expr) => {
        match $m {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("Database Manager returned error: {}", e);
                return Ok(json_response!(
                        status: $status,
                        body: &e.to_string()))
            }
        }
    }
}

pub async fn get_url_maps(req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<Message>>().unwrap();
    sender_failed_json!(
        sender
        .send(Message::GetUrlMaps { resp: tx })
        .await, "GetUrlMaps");
    let url_maps = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::INTERNAL_SERVER_ERROR);
    Ok(json_response!(body: &url_maps))
}

pub async fn get_url_map(req: Request<Body>) -> Result<Response<Body>> {
    let sender = req.data::<Sender<Message>>().unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    let key = req.param("key").unwrap();
    sender_failed_json!(
        sender
        .send(Message::GetUrlMap { key: key.into(), resp: tx })
        .await, "GetUrlMap");
    let url_map = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::NOT_FOUND);
    Ok(json_response!(body: &url_map))
}

pub async fn create_url_map(mut req: Request<Body>) -> Result<Response<Body>> {
    let body = req.body_mut();
    let url_map_bytes = to_bytes(body).await?;
    let url_map = serde_json::from_slice::<UrlMap>(&url_map_bytes)?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<Message>>().unwrap();
    sender_failed_json!(
        sender
        .send(Message::CreateUrlMap { url_map, resp: tx })
        .await, "CreateUrlMap");
    let url_map = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::UNPROCESSABLE_ENTITY);
    Ok(json_response!(body: &url_map))
}

pub async fn update_url_map(mut req: Request<Body>) -> Result<Response<Body>> {
    #[derive(Debug, Serialize, Deserialize)]
    struct UrlMapUrl {
        url: String,
    }

    let body = req.body_mut();
    let url_map_url_bytes = to_bytes(body).await?;
    let url_map_url = serde_json::from_slice::<UrlMapUrl>(&url_map_url_bytes)?;
    let key = req.param("key").unwrap();
    let url_map = UrlMap::new(key.into(), url_map_url.url);
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<Message>>().unwrap();
    sender_failed_json!(
        sender
        .send(Message::UpdateUrlMap { url_map, resp: tx })
        .await, "UpdateUrlMap");
    let url_map = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::UNPROCESSABLE_ENTITY);
    Ok(json_response!(body: &url_map))
}

pub async fn delete_url_map(req: Request<Body>) -> Result<Response<Body>> {
    let key = req.param("key").unwrap();
    let sender = req.data::<Sender<Message>>().unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    sender_failed_json!(
        sender
        .send(Message::DeleteUrlMap { key: key.into(), resp: tx })
        .await, "DeleteUrlMap");
    recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::NOT_FOUND);
    Ok(json_response!(body: &serde_json::json!({
        "ok": "true"
    }).to_string()))
}
