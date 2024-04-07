#[allow(dead_code)]
mod proxy_dev;

use std::env;
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
use std::process::exit;
use log::{error, info};
use tokio::net::TcpListener;

extern crate pretty_env_logger;

fn main() {
    // Accept commandline parameters for proxy_stream and origin_stream
    pretty_env_logger::init();

    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        error!("missing proxy address");
        exit(2);
    }

    let proxy_server = &args[2];
    info!("starting server on {}",proxy_server);

    // // Start a socket server on proxy_stream
    let proxy_listener;
    if let Ok(proxy) = TcpListener::bind(proxy_server) {
        proxy_listener = proxy;
    } else {
        error!("failed to bind to {}", proxy_server);
        exit(2);
    }

    


}
