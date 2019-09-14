use http;
use hyper::{self};
use serde_json::Error as SerdeError;
use std::{io::Error as IoError};

#[derive(Debug)]
pub enum FetchError {
    SerdeJsonError(SerdeError),
    Hyper(hyper::Error),
    Http(http::Error),
    IO(IoError)
}

impl From<SerdeError> for FetchError {
    fn from(error: SerdeError) -> FetchError {
        FetchError::SerdeJsonError(error)
    }
}

impl From<hyper::Error> for FetchError {
    fn from(error: hyper::Error) -> FetchError {
        FetchError::Hyper(error)
    }
}

impl From<http::Error> for FetchError {
    fn from(error: http::Error) -> FetchError {
        FetchError::Http(error)
    }
}

impl From<IoError> for FetchError {
    fn from(error: IoError) -> FetchError {
        FetchError::IO(error)
    }
}