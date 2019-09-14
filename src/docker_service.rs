use hyper::Client;
use hyper::rt::{Stream};
use serde::de::DeserializeOwned;
use hyperlocal::{UnixConnector, Uri};
use tokio::prelude::Future;
use crate::structs::{Container, State};

pub fn get_missing_containers(desired_state: State) -> impl Future<Item=State, Error=FetchError> {
    get_running_containers().map(| running_containers| {
        let mut containers = running_containers.into_iter();

        State {
            containers: desired_state.containers.into_iter().filter(|desired_container| {
                containers.all(|running_container| running_container.image != desired_container.image)
            }).collect()
        }
    })
}

pub fn get_running_containers() -> impl Future<Item=Vec<Container>, Error=FetchError> {
    fetch_docker_url::<Vec<Container>>("/v1.40/containers/json?filters=%7B%22status%22%3A%7B%22running%22%3Atrue%7D%7D")
        .map(|containers: Vec<Container>| {
            containers.into_iter().filter(|container| {
                container.labels.contains_key("me.maartedev.simplekube")
            }).collect()
        })
}

fn fetch_docker_url<T: DeserializeOwned>(path: &str) -> impl Future<Item=T, Error=FetchError> {
    let client = Client::builder().keep_alive(false)
        .build::<_, ::hyper::Body>(UnixConnector::new());
    let url = Uri::new("/var/run/docker.sock", path).into();

    client
        .get(url)
        .and_then(|res| {
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let response = serde_json::from_slice(&body)?;

            Ok(response)
        })
        .from_err()
}

#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}