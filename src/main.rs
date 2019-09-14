extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;
extern crate serde_aux;

use tokio::prelude::Future;
use crate::structs::{State, Container, DesiredContainer};

mod structs;
mod docker_service;
mod api_service;

fn main() {
    let desired_state = State {
        containers: vec![
            DesiredContainer {
                image: "nginxdemos/hello:plain-text".to_owned()
            },
            DesiredContainer {
                image: "nginxdemos/hello".to_owned()
            }
        ]
    };

    let future = docker_service::get_missing_containers(desired_state)
        .map(|state| {
            println!("missing: {:#?}", state)
        }).map_err(|e| eprintln!("Error: {:#?}", e));

    tokio::run(future)
//    api_service::run();
}

