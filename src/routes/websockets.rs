use actix_web::{web, HttpResponse};
use actix_ws::{Message, Websocket};
use tokio::sync::mpsc;
use crate::models::sensor::SensorData;
use std::sync::Arc;

pub async fn ws_endpoint(ws: Websocket, state: web::Data<Arc<SensorData>>) {
    let (mut tx, mut rx) = ws.split();

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        let msg = serde_json::to_string(&*state).unwrap();
        if tx.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}

