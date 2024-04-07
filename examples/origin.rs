use std::env;
use std::fmt::format;
use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use tokio::net::TcpListener;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use hyper_util::rt::TokioIo;


type Counter = i32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<_> = env::args().collect();
    let addr = &args[1];

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let svc = Svc {
        counter: Arc::new(Mutex::new(0)),
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let svc_clone = svc.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

#[derive(Debug, Clone)]
struct Svc {
    counter: Arc<Mutex<Counter>>,
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        let uri_path = req.uri().path();

        self.count();

        let body = format!("count: {:?}", *self.counter);

        let res = match req.uri().path() {
            "/test" => mk_response(format!("test! counter = {:?}", self.counter)),
            _ => return Box::pin(async { mk_response(body.into()) })
        };

        Box::pin(async { res })
    }
}

impl Svc {
    fn count(&self) {
        let mut counter = self.counter.lock().expect("lock poisioned");
        *counter += 1;
    }
}
