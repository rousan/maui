extern crate maui_server;
use hyper::Server;
use maui_server::{constants, routes, startup, utils, ResultExt};
use routerify::RouterService;
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() {
    startup::up().await.context("Failed to startup the server").unwrap();

    let addr = SocketAddr::new(
        utils::env(constants::env::HOST)
            .and_then(|host| host.parse::<IpAddr>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_IP),
        utils::env(constants::env::PORT)
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_PORT),
    );

    let server = Server::bind(&addr)
        .http1_keepalive(true)
        .http1_half_close(true)
        .http1_only(false)
        .http2_only(false)
        .http1_writev(true)
        .tcp_sleep_on_accept_errors(true)
        .serve(RouterService::new(routes::router()).unwrap());

    maui_server::info!("App is serving on: {}", server.local_addr());
    if let Err(e) = server.await {
        maui_server::error!("Server Error: {}", e);
    }
}