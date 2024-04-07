#[allow(dead_code)]
mod proxy_dev;

use std::env;
// use std::net::TcpStream;
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
use std::process::exit;
use log::{error, info, trace};
// use tokio::io::{AsyncReadExt,AsyncWriteExt};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
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

    for proxy_stream in proxy_listener.incoming() {
        let mut proxy_stream = proxy_stream.expect("Error in incoming TCP connection");

        let mut in_buffer: Vec<u8> = vec![0; 200];
        if let Err(err) = proxy_stream.read(&mut in_buffer) {
            error!("error in reading from incoming proxy stream: {}", err);
            exit(2);
        }

        let request_string = String::from_utf8_lossy(&in_buffer);
        info!("accepting incoming client request \n{}",request_string);

        let request_lines: Vec<&str> = request_string.split("\r\n").collect();

        if let Some(first_line) = request_lines.first() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();

            if parts.len() >= 2 {
                let path = parts[1];
                trace!("request path: {}", path);
                // Now you can use the path as needed
            }
        } else {
            trace!("could not split the request text")
        }

    }



        // let mut thread_handles = Vec::new();

}

// fn handle_conn(proxy_stream: &mut TcpStream, origin_stream: &mut TcpStream) {}