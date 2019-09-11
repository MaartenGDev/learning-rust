extern crate futures;

use hyper::service::service_fn;
use futures::{future, Future};
use hyper::{Method, StatusCode, Response, Body, Request, Server};
use hyper::rt::Stream;

fn main() {
    type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

    fn echo(req: Request<Body>) -> BoxFut {
        let mut response = Response::new(Body::empty());

        match (req.method(), req.uri().path()) {
            (&Method::GET, "/services") => {
                *response.body_mut() = Body::from("Try POSTing data to /echo");
            },
            (&Method::POST, "/echo") => {
                let mapping = req
                    .into_body()
                    .map(|chunk| {
                        chunk.iter()
                            .map(|byte| byte.to_ascii_uppercase())
                            .collect::<Vec<u8>>()
                    });

            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            },
        };

        Box::new(future::ok(response))
    }

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

