#[allow(dead_code)]
#[allow(unused_imports)]

mod proxy_dev;

use std::{env, thread};
use std::fmt::{Display, Formatter, Pointer};
// use std::net::TcpStream;
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
use std::process::exit;
use log::{error, info, trace};
// use tokio::io::{AsyncReadExt,AsyncWriteExt};
use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use form_urlencoded::parse;
use hyper::Method;

extern crate pretty_env_logger;

enum RequestMethod {
    GET,
    POST,
}

struct Request {
    method: String,
    http_version: String,
    route: String
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} HTTP {}", self.method, self.route, self.http_version)
    }
}

#[tokio::main]
async fn main() {
    // Accept commandline parameters for proxy_stream and origin_stream
    pretty_env_logger::init();

    // let args: Vec<_> = env::args().collect();
    //
    // if args.len() < 3 {
    //     error!("missing proxy address");
    //     exit(2);
    // }
    //
    // let proxy_server = &args[2];
    let proxy_server = "127.0.0.1:8000";
    info!("starting server on {}",proxy_server);

    let proxy_listener;

    // Start a socket server on proxy_stream
    match TcpListener::bind(proxy_server) {
        Ok(proxy) => { proxy_listener = proxy}
        Err(err) => {
            error!("failed to bind to: {} \n error: {}", proxy_server, err);
            exit(2);
        }
    }

    let mut thread_handles = Vec::new();

    for proxy_stream in proxy_listener.incoming() {
        let mut proxy_stream = proxy_stream.expect("error in incoming TCP connection");

        let handle = thread::spawn(move || handle_connection(&mut proxy_stream));

        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().expect("Unable to join child thread");
    }
}

fn handle_connection(income_stream: &mut TcpStream) {
    let mut in_buffer: Vec<u8> = vec![0; 200];

    if let Err(err) = income_stream.read(&mut in_buffer) {
        error!("error in reading from incoming proxy stream: {}", err);
        exit(2);
    }

    let mut request_string = String::from_utf8_lossy(&in_buffer).to_string();

    // terminate everything else,
    // \r\n\r\n determines termination of the request
    if let Some(index) = request_string.find("\r\n\r\n") {
        request_string.truncate(index);
    }

    let request = parse_request(request_string);
    info!("request \n{}",request);
    // let request_lines : Vec<&str> = request_string.split("\n").collect();
    // trace!("lines count {} \n request lines {:#?} \n could not find anything", request_lines.len(), request_lines);
}

fn parse_request(request: String) -> Request{
    // let request = request.lines();

    let request_lines: Vec<&str> = request.split("\n").collect();

    let first_line = request_lines[0];
    let mut parts = first_line.split(" ");

    let method = parts.next().unwrap().to_string();
    let route = parts.next().unwrap().to_string();
    let http_version = parts.next().unwrap().to_string();

    Request {
        method,
        http_version,
        route,
    }
}

// fn handle_conn(proxy_stream: &mut TcpStream, origin_stream: &mut TcpStream) {}

// if let Some(first_line) = request_lines.first() {
//     let parts: Vec<&str> = first_line.split_whitespace().collect();
//
//     if parts.len() >= 2 {
//         let path = parts[1];
//         trace!("request path: {}", path);
//         // Now you can use the path as needed
//     }
// } else {
//     trace!("could not split the request text")
// }