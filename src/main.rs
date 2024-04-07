#[allow(dead_code)]
#[allow(unused_imports)]
mod proxy_dev;

use tokio::task;

use std::{env, thread};
use std::fmt::{Display, Formatter, Pointer};
// use std::net::TcpStream;
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
use std::process::exit;
use log::{error, info, trace};
// use tokio::io::{AsyncReadExt,AsyncWriteExt};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
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
    route: String,
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
        Ok(proxy) => { proxy_listener = proxy }
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
        handle.join().expect("unable to join child thread");
    }
}

fn handle_connection(proxy_stream: &mut TcpStream) {
    let mut in_buffer: Vec<u8> = vec![0; 200];
    // let mut proxy_stream = proxy_stream;

    if let Err(err) = proxy_stream.read(&mut in_buffer) {
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

    let origin_server = match request.route.as_str() {
        "/server1" => "127.0.0.1:1337",
        "/server2" => "127.0.0.1:1338",
        _ => "127.0.0.1:1330"
    };

    trace!("origin server {}",origin_server);


    match TcpStream::connect(origin_server) {
        Ok(mut origin_stream) => {
            let mut out_buffer: Vec<u8> = vec![0; 200];

            let _ = origin_stream.write(&mut in_buffer).unwrap();
            trace!("2: Forwarding request to origin server\n");

            // Read response from the backend server
            let _ = origin_stream.read(&mut out_buffer).unwrap();
            
            trace!( "3: Received response from origin server: {}",
                String::from_utf8_lossy(&out_buffer)
            );

            // Write response back to the proxy client
            let _ = proxy_stream.write(&mut out_buffer).unwrap();
            trace!("4: Forwarding response back to client");
        }
        Err(err) => {
            error!("failed to connect to origin server\n{}",err)
        }
    }
}

fn parse_request(request: String) -> Request {
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
