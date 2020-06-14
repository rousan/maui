use crate::prelude::*;
use hyper::{Body, Request, Response};
use routerify_websocket::WebSocket;

pub async fn ws_handler(ws: WebSocket) {
    info!("New WebSocket connection from: {}", ws.remote_addr());

    todo!()
}
