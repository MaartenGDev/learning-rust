extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;
extern crate serde_aux;
extern crate mime;
extern crate redis;
extern crate r2d2_redis;

use std::thread;

mod structs;
mod docker_client;
mod docker_service;
mod api_service;
mod errors;
mod scheduling_service;

fn main() -> redis::RedisResult<()> {
    thread::spawn(move || {
        scheduling_service::run().unwrap();
    });
    api_service::run();

    Ok(())
}

