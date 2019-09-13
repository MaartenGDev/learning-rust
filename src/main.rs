extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;
extern crate serde_aux;

mod docker_service;
mod api_service;

fn main() {
    docker_service::run();
//    api_service::run();
}

