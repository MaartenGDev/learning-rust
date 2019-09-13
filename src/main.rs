extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;

mod docker_service;
mod api_service;

fn main() {
    docker_service::run();
//    api_service::run();
}

