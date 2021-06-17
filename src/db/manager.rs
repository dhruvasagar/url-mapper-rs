use crate::db::{DB, UrlMap};
use sqlx::{Postgres, pool::PoolConnection};
use tokio::sync::{mpsc::Receiver, oneshot::Sender};

type Responder<T> = Sender<Result<T, sqlx::Error>>;

#[derive(Debug)]
pub enum Message {
    GetUrlMaps { resp: Responder<Vec<UrlMap>> },
    GetUrlMap { key: String, resp: Responder<UrlMap> },
    CreateUrlMap { url_map: UrlMap, resp: Responder<UrlMap> },
    UpdateUrlMap { url_map: UrlMap, resp: Responder<UrlMap> },
    DeleteUrlMap { key: String, resp: Responder<UrlMap> },
}

pub struct Manager {
    db: DB,
    receiver: Receiver<Message>,
}

type Connection = PoolConnection<Postgres>;

macro_rules! resp_failed {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {},
            Err(e) => eprintln!("Resp failed for {}, error: {:?}", $f, e)
        }
    }
}

impl Manager {
    pub fn new(db: DB, receiver: Receiver<Message>) -> Self {
        Self { db, receiver }
    }

    async fn get_url_maps(conn: &mut Connection) -> Result<Vec<UrlMap>, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("SELECT * FROM url_maps")
            .fetch_all(conn)
            .await
    }

    async fn get_url_map(conn: &mut Connection, key: String) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("SELECT * FROM url_maps WHERE key = $1")
            .bind(key)
            .fetch_one(conn)
            .await
    }

    async fn create_url_map(conn: &mut Connection, url_map: UrlMap) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("INSERT INTO url_maps (key, url) VALUES ($1, $2) RETURNING *")
            .bind(url_map.key)
            .bind(url_map.url)
            .fetch_one(conn)
            .await
    }

    async fn update_url_map(conn: &mut Connection, url_map: UrlMap) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("UPDATE url_maps SET url=$1 WHERE key=$2 RETURNING *")
            .bind(url_map.url)
            .bind(url_map.key)
            .fetch_one(conn)
            .await
    }

    async fn delete_url_map(conn: &mut Connection, key: String) -> Result<UrlMap, sqlx::Error> {
        sqlx::query_as::<_, UrlMap>("DELETE FROM url_maps WHERE key = $1 RETURNING *")
            .bind(key)
            .fetch_one(conn)
            .await
    }

    pub async fn listen(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            let mut connection = self.db.pool.acquire().await.unwrap();
            match message {
                Message::GetUrlMaps { resp } => {
                    let url_maps = Self::get_url_maps(&mut connection).await;
                    resp_failed!(resp.send(url_maps), "GetUrlMaps");
                }
                Message::GetUrlMap { key, resp } => {
                    let url_map = Self::get_url_map(&mut connection, key).await;
                    resp_failed!(resp.send(url_map), "GetUrlMap");
                }
                Message::CreateUrlMap { url_map, resp } => {
                    let url_map = Self::create_url_map(&mut connection, url_map).await;
                    resp_failed!(resp.send(url_map), "CreateUrlMap");
                }
                Message::UpdateUrlMap { url_map, resp } => {
                    let url_map = Self::update_url_map(&mut connection, url_map).await;
                    resp_failed!(resp.send(url_map), "UpdateUrlMap");
                }
                Message::DeleteUrlMap { key, resp } => {
                    let url_map = Self::delete_url_map(&mut connection, key).await;
                    resp_failed!(resp.send(url_map), "DeleteUrlMap");
                }
            }
        }
    }
}
