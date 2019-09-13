use hyper::Client;
use hyper::rt::{self, Future, Stream};
use serde::{Deserialize};
use serde::de::{DeserializeOwned};
use hyperlocal::{UnixConnector, Uri};

pub fn run() {
    let fut= fetch_docker_url("/v1.24/containers/json?all=1")
        .map(|containers: Vec<Container>| {
            println!("containers: {:#?}", containers);
        })
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });

    rt::run(fut);
}

fn fetch_docker_url<T>(path: &str) -> impl Future<Item=Vec<T>, Error=FetchError> where T: DeserializeOwned{
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
            let users = serde_json::from_slice(&body)?;

            Ok(users)
        })
        .from_err()
}

#[derive(Deserialize, Debug)]
struct Container {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Image")]
    image: String,
}

enum FetchError {
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