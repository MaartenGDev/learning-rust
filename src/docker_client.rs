use hyper::{Client, Body, Request, Method, header};
use hyper::rt::{Stream};
use serde::de::DeserializeOwned;
use hyperlocal::{UnixConnector, Uri};
use tokio::prelude::Future;
use crate::structs::{Container, DesiredContainer, ContainerCreated};
use crate::errors::FetchError;
use serde::export::fmt::Debug;
use serde_json::json;

pub fn get_running_containers() -> impl Future<Item=Vec<Container>, Error=FetchError> {
    get_json::<Vec<Container>>("/v1.40/containers/json?filters=%7B%22status%22%3A%7B%22running%22%3Atrue%7D%7D")
}

pub fn create_container(desired_container: &DesiredContainer) -> impl Future<Item=ContainerCreated, Error=FetchError> {
    let container_json = json!({
            "Image": desired_container.image.to_string(),
            "Labels": {
               "me.maartedev.simplekube": "1.0"
            },
            "Ports": {
                "80/tcp": [
                    {
                        "HostIp": "0.0.0.0",
                        "HostPort": "0"
                    }
                ]
            }
        });

    post_json::<ContainerCreated>("/containers/create", container_json.to_string()).map(|option| {
        match option {
            Some(x) => x,
            None => panic!("Failed to create container")
        }
    })
}

pub fn start_container(created_container: &ContainerCreated) -> impl Future<Item=bool, Error=FetchError> {
    post_json::<bool>(format!("/containers/{}/start", created_container.id).as_str(), "".to_owned()).map(|option|  {
        match option {
            _ => true
        }
    })
}


fn get_json<T: DeserializeOwned>(path: &str) -> impl Future<Item=T, Error=FetchError> {
    let url = Uri::new("/var/run/docker.sock", path).into();

    build_client()
        .get(url)
        .and_then(|res| {
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let response = serde_json::from_slice(&body)?;
            Ok(response)
        })
}

fn post_json<T: DeserializeOwned + Debug>(path: &str, data: String) -> impl Future<Item=Option<T>, Error=FetchError> {
    let url: hyper::Uri = Uri::new("/var/run/docker.sock", path).into();

    let request_builder = Request::builder()
        .method(Method::POST)
        .uri(url.to_string())
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .body(data.into());

    let request = match request_builder {
        Ok(result) => result,
        Err(e) => panic!("Failed to create request, {}", e),
    };


    build_client()
        .request(request)
        .and_then(|res| {
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            Ok(match body.is_empty(){
                true => None::<T>,
                false => Some(serde_json::from_slice(&body).unwrap())
            })
        })
}

fn build_client() -> Client<UnixConnector, Body> {
    Client::builder().keep_alive(false)
        .build::<_, ::hyper::Body>(UnixConnector::new())
}
