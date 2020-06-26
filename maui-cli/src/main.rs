extern crate maui;
use clap::{App, Arg};
use colored::*;
use maui::constants;
use maui::help;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let matches = App::new(constants::APP_NAME)
        .version(constants::APP_VERSION)
        .version_short("v")
        .author(constants::APP_AUTHOR)
        .about(constants::APP_DESCRIPTION)
        .after_help(help::EXAMPLES_HELP_TEXT)
        .arg(
            Arg::with_name("target_address")
                .help("The tunneling target address")
                .value_name("HOST:PORT or PORT")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("api_server")
                .short("s")
                .long("api-server")
                .value_name("HOST:PORT")
                .help("Specify the API server host and port to use instead of the default one")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("is_secure")
                .long("secure")
                .help("Specify whether to TLS or not [UPCOMING]")
                .required(false),
        )
        .get_matches();

    let target_address = matches.value_of("target_address").unwrap();
    let api_server = matches.value_of("api_server").unwrap_or(constants::DEFAULT_API_SERVER);
    let is_secure = matches.is_present("is_secure");

    if let Err(err) = maui::tunnel::tunnel_address(target_address, api_server, is_secure).await {
        eprintln!("{} {}", "error:".red(), err);
    }
}
