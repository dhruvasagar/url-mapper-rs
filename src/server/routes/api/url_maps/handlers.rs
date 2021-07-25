use anyhow::Result;
use serde::{Serialize, Deserialize};
use hyper::{Body, Request, Response, body::to_bytes};
use routerify::ext::RequestExt;
use crate::{db::{UrlMap, Message}, server::State};

pub async fn get_url_maps(req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
    sender_failed_json!(
        sender
        .send(Message::GetUrlMaps { resp: tx })
        .await, "GetUrlMaps");
    let url_maps = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::INTERNAL_SERVER_ERROR);
    Ok(json_response!(body: &url_maps))
}

pub async fn get_url_map(req: Request<Body>) -> Result<Response<Body>> {
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
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
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
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
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
    sender_failed_json!(
        sender
        .send(Message::UpdateUrlMap { url_map, resp: tx })
        .await, "UpdateUrlMap");
    let url_map = recv_failed_json!(rx.await.unwrap(), hyper::StatusCode::UNPROCESSABLE_ENTITY);
    Ok(json_response!(body: &url_map))
}

pub async fn delete_url_map(req: Request<Body>) -> Result<Response<Body>> {
    let key = req.param("key").unwrap();
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
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
