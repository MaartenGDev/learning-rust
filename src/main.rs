extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;
extern crate serde_aux;
extern crate mime;

use futures::future::lazy;
use tokio::prelude::Future;
use crate::structs::{State, Container, DesiredContainer};

mod structs;
mod docker_client;
mod docker_service;
mod api_service;
mod errors;

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

    tokio::run(lazy(|| {
        let future = docker_service::get_missing_containers(desired_state)
            .map(|state| {
                println!("missing: {:#?}", state);

                for container in state.containers {
                    let future = docker_service::schedule_container(&container)
                        .map(|start_container| {
                            tokio::spawn(start_container.map(|_| {
                                println!("Started!");
                            }).map_err(|e| eprintln!("Error: {:#?}", e)));
                        })
                        .map_err(|e| eprintln!("Error: {:#?}", e));

                    tokio::spawn(future);
                }
            }).map_err(|e| eprintln!("Error: {:#?}", e));


        tokio::spawn(future);
        Ok(())
    }));

//    api_service::run();
}

