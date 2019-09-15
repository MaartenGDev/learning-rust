use futures::future::lazy;
use tokio::prelude::Future;
use crate::structs::{State};
use crate::docker_service;

pub fn run<F>(mut fetcher: F) where F: FnMut() -> redis::RedisResult<State> {
    let desired_state = fetcher();

    match desired_state {
        Err(e) => panic!("No state found!, {}", e),
        Ok(state) => {
            tokio::run(lazy(|| {
                let future = docker_service::get_missing_containers(state)
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
        }
    };
}

