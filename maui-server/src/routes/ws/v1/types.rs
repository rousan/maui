use crate::prelude::*;
use routerify_websocket::Message;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    pub counter: i64,
}

impl TryFrom<WsMessage> for Message {
    type Error = crate::Error;

    fn try_from(msg: WsMessage) -> Result<Self, Self::Error> {
        Message::json(&msg).context("Failed to convert a `WsMessage` to `Message`")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewConnectionOutput {
    pub id: String,
    pub ip_address: String,
}
