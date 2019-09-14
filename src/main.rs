extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;
extern crate serde_aux;
extern crate mime;
extern crate redis;

use tokio::prelude::Future;
use crate::structs::{State, DesiredContainer};
use redis::{Commands, RedisResult};

mod structs;
mod docker_client;
mod docker_service;
mod api_service;
mod errors;
mod scheduling_service;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://0.0.0.0/")?;
    let mut con = client.get_connection()?;

    let desired_state_key = "desired_state";

    if !con.exists(desired_state_key)? {
        con.set(desired_state_key, "[]")?;
    }

    let mut fetcher = || -> redis::RedisResult<State>{
        let desired_state: String = con.get(desired_state_key)?;

        Ok(State {
            containers: vec![
                DesiredContainer {
                    image: "nginxdemos/hello:plain-text".to_owned()
                },
                DesiredContainer {
                    image: "nginxdemos/hello".to_owned()
                }
            ]
        })
    };


    let mut set_state = | next_state: String| -> redis::RedisResult<()>{
//        con.set(desired_state_key, next_state)?;

        Ok(())
    };




    scheduling_service::run(fetcher);
    api_service::run(set_state);

    Ok(())
}

