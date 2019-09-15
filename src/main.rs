extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;
extern crate serde_aux;
extern crate mime;
extern crate redis;
extern crate r2d2_redis;

use crate::structs::{State};
use redis::{Commands};

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
        con.set(desired_state_key, r#"{"containers": []}"#)?;
    }

    let fetcher = || -> redis::RedisResult<State>{
        let raw_json: String = con.get(desired_state_key)?;
        println!("{}", &raw_json);
        let desired_state: State = serde_json::from_str(&raw_json).unwrap();

        Ok(desired_state)
    };

    scheduling_service::run(fetcher);
    api_service::run();

    Ok(())
}

