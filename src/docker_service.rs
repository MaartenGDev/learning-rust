use hyperlocal::{Uri, UnixConnector};
use std::io::{self, Write};
use futures::Stream;
use futures::Future;
use hyper::{Client, rt, Body};
use futures::future::Err;
use std::error::Error;
use itertools::Itertools;
use serde_json::Value;

pub fn run() {
    send_request_docker_request("/v1.24/containers/json?all=1");
}

fn send_request_docker_request(path: &str) ->  ::std::result::Result<(), Box<dyn Error>> {
    let client = Client::builder().keep_alive(false)
        .build::<_, ::hyper::Body>(UnixConnector::new());
    let url = Uri::new("/var/run/docker.sock", path).into();


    let work = client
        .get(url)
        .and_then(|res| {
            let oef: Vec<u8> = res.into_body()
                .fold(Vec::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&*chunk);
                    futures::future::ok::<_, hyper::Error>(acc)
                }).wait().unwrap();

            let v: Vec<Value> = serde_json::from_str(String::from_utf8(oef).unwrap().as_str()).expect("Received invalid json");

            Ok(())
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        });


    rt::run(work);

    Ok(())
}

