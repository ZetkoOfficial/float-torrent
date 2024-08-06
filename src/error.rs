pub mod error {
    use std::{io, result};

    use serde::Serialize;
    use tokio::net::TcpStream;

    use crate::http;

    #[derive(Debug, Serialize)]
    #[allow(dead_code)]
    pub enum Error {
        IOError(ErrorResponse),
        HttpParseError(ErrorResponse),
        SerdeError(ErrorResponse),
        HttpRequestTooShort(ErrorResponse),
        MissingPath(ErrorResponse),
    }

    #[derive(Debug, Serialize)]
    pub struct ErrorResponse {
        message: String,
        extra: Option<serde_json::Value>
    }

    impl ErrorResponse {
        // panic, če extra_json ni veljaven
        pub fn new_with_extra(message: &str, extra_json: &str) -> Self {
            ErrorResponse { message: message.to_owned(), extra: Some(serde_json::from_str(extra_json).unwrap()) }
        }
    }

    impl From<&str> for ErrorResponse {
        fn from(value: &str) -> Self {
            ErrorResponse { message: value.to_string(), extra: None }
        }
    }

    pub type Result<T> = result::Result<T, Error>;
    impl From<io::Error> for Error {
        fn from(value: io::Error) -> Self {
            Error::IOError(ErrorResponse { message: value.to_string(), extra: None })
        }
    }
    impl From<httparse::Error> for Error {
        fn from(value: httparse::Error) -> Self {
            Error::HttpParseError(ErrorResponse { message: value.to_string(), extra: None })
        }
    }

    impl From<serde_json::Error> for Error {
        fn from(value: serde_json::Error) -> Self {
            Error::SerdeError(ErrorResponse { message: value.to_string(), extra: None })
        }
    }

    impl Error {
        fn as_sendable(&self) -> Result<Vec<u8>> {
            Ok(serde_json::to_vec_pretty(&self)?)
        }

        // če je mogoče vrne error, drugače samo preskočimo
        pub async fn send_error(self, stream: &mut TcpStream) {
            http::write::write_http("400 Bad Request", &self.as_sendable().unwrap_or_default(), stream).await.unwrap_or_default()
        } 
    }
}

