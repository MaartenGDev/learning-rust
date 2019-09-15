use futures::future::lazy;
use tokio::prelude::Future;
use crate::structs::State;
use crate::docker_service;
use redis::Commands;
use std::{thread, time};

pub fn run() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://0.0.0.0/").unwrap();
    let mut con = client.get_connection().unwrap();

    let desired_state_key = "desired_state";
    let has_previous_value: bool = con.exists(desired_state_key).unwrap();

    if !has_previous_value {
        con.set(desired_state_key, r#"{"containers": []}"#)?
    }

    loop {
        let raw_json: String = con.get(desired_state_key).unwrap();
        let desired_state: State = serde_json::from_str(&raw_json).unwrap();

        tokio::run(lazy(move || {
            start_missing_containers(desired_state.clone());
            stop_zombie_containers(desired_state.clone());

            Ok(())
        }));

        thread::sleep(time::Duration::from_secs(4));
    }
}

fn start_missing_containers(state: State) {
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
}


fn stop_zombie_containers(state: State) {
    let future = docker_service::get_zombie_containers(state)
        .map(|state| {
            println!("zombies: {:#?}", state);

            for zombie_container in state.containers {
                let future = docker_service::stop_container(&zombie_container)
                    .map(|_| {
                        println!("Stopped container!");
                    })
                    .map_err(|e| eprintln!("Error: {:#?}", e));

                tokio::spawn(future);
            }
        }).map_err(|e| eprintln!("Error: {:#?}", e));

    tokio::spawn(future);
}
