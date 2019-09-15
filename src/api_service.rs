use hyper::service::service_fn;
use futures::{future, Future};
use hyper::{Method, StatusCode, Response, Body, Request, Server, header};
use hyper::rt::Stream;
use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;
use crate::structs::State;
use r2d2_redis::r2d2::Pool;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=GenericError> + Send>;

fn store_desired_state(req: Request<Body>, pool: &Pool<RedisConnectionManager>) -> ResponseFuture {
    let mut connection = pool.get().unwrap();

    Box::new(req.into_body()
        .concat2()
        .from_err()
        .and_then(move |entire_body| {
            let body_as_string = String::from_utf8(entire_body.to_vec()).unwrap();
            let desired_state: State = serde_json::from_str(&body_as_string).unwrap();
            let _: String = connection.set("desired_state", serde_json::to_string(&desired_state).unwrap()).unwrap();

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"success": true}"#)).unwrap();

            Ok(response)
        }))
}

fn router(req: Request<Body>, pool: &Pool<RedisConnectionManager>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/state") => {
            store_desired_state(req, pool)
        }
        _ => {
            // Return 404 not found response.
            let body = Body::empty();
            Box::new(future::ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap()))
        }
    }
}


pub fn run() {
    let addr = "127.0.0.1:3000".parse().unwrap();


    hyper::rt::run(future::lazy(move || {
        let manager = RedisConnectionManager::new("redis://0.0.0.0").unwrap();
        let pool = r2d2::Pool::builder().build(manager).unwrap();

        let new_service = move || {
            let pool = pool.clone();

            service_fn(move |req: Request<Body>| {
                router(req, &pool)
            })
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        server
    }));
}