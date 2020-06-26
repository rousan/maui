use crate::prelude::*;
use std::net::SocketAddr;
use tokio::time::{self, Duration};

pub async fn tunnel_address(target_address: &str, api_server: &str, _is_secure: bool) -> crate::Result<()> {
    let target_address = if target_address.contains(":") {
        target_address.parse::<SocketAddr>().context("Invalid host and port")?
    } else {
        let port = target_address.parse::<u16>().context("Invalid port")?;
        SocketAddr::new("127.0.0.1".parse().unwrap(), port)
    };

    let api_server = api_server.parse::<SocketAddr>().context("Invalid API server address")?;

    // let a = start_daemon_ws_conn()

    Ok(())
}

pub async fn start_daemon_ws_conn(api_server: SocketAddr) {
    while let Err(err) = try_ws_conn(api_server).await {
        println!("WebSocket connection is closed, trying again");
        time::delay_for(Duration::from_secs(10)).await;
    }
}

pub async fn try_ws_conn(api_server: SocketAddr) -> crate::Result<()> {
    let (mut ws, _) = tokio_tungstenite::connect_async(constants::API_SERVER_ENDPOINT_WS_CONNECT)
        .await
        .context("Failed to create WebSocket connection from the API server")?;

    bridge::send_message(
        what::SNACK_BAR_MSG,
        MessageData::new().put_string("msg", "Connected with API server"),
    );

    counter::fetch_and_publish_counter_value().await;

    while let Some(msg) = ws.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                let _ = ws.close(None).await;
                return Err(err.context("Failed to get next WS message"));
            }
        };

        if !msg.is_text() {
            continue;
        }

        let msg = match WsMessage::try_from(msg) {
            Ok(msg) => msg,
            Err(err) => {
                let _ = ws.close(None).await;
                return Err(err.context("Failed to parse WS message"));
            }
        };

        publish_counter_value(msg.counter);
    }

    Ok(())
}
