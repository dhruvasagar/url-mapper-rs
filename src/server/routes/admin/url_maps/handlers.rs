use crate::{db::Message, server::State};
use hyper::{Body, Request, Response};
use anyhow::Result;
use routerify::ext::RequestExt;
use tera::Context;

pub async fn index(req: Request<Body>) -> Result<Response<Body>> {
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
    let tera = state.tera();

    let (tx, rx) = tokio::sync::oneshot::channel();
    sender_failed!(
        sender
        .send(Message::GetUrlMaps { resp: tx })
        .await, "GetUrlMaps");
    let url_maps = recv_failed!(rx.await.unwrap());

    let mut context = Context::new();
    context.insert("url_maps", &url_maps);
    let index_html = tera.render("url_maps/index.html", &context)?;

    Ok(Response::builder()
       .body(Body::from(index_html))
       .unwrap())
}

pub async fn new(req: Request<Body>) -> Result<Response<Body>> {
    let state = req.data::<State>().unwrap();
    let tera = state.tera();

    let new_html = tera.render("url_maps/new.html", &Context::new())?;

    Ok(Response::builder()
       .body(Body::from(new_html))
       .unwrap())
}

pub async fn edit(req: Request<Body>) -> Result<Response<Body>> {
    let state = req.data::<State>().unwrap();
    let sender = state.db_sender();
    let tera = state.tera();
    let key = req.param("key").unwrap();

    let (tx, rx) = tokio::sync::oneshot::channel();
    sender_failed!(
        sender
        .send(Message::GetUrlMap { key: key.into(), resp: tx })
        .await, "GetUrlMap");
    let url_map = recv_failed!(rx.await.unwrap());

    let mut context = Context::new();
    context.insert("url_map", &url_map);
    let edit_html = tera.render("url_maps/edit.html", &context)?;

    Ok(Response::builder()
       .body(Body::from(edit_html))
       .unwrap())
}
