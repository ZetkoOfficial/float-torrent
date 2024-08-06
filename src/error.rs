pub mod error {
    use std::{io, result};

    use serde::Serialize;
    use tokio::net::TcpStream;

    use crate::http;

    #[derive(Debug, Serialize)]
    #[allow(dead_code)]
    pub enum ErrorType {
        IOError,
        HttpParseError,
        SerdeError,
        HttpRequestTooShort,
        MissingPath,
    }

    #[derive(Debug, Serialize)]
    pub struct Error {
        error_type: ErrorType,
        message:    String,
        extra:      Option<serde_json::Value>
    }

    pub type Result<T> = result::Result<T, Error>;
    impl From<io::Error> for Error {
        fn from(value: io::Error) -> Self {
            Error {
                error_type: ErrorType::IOError,
                message: value.to_string(),
                extra: None
            }
        }
    }
    impl From<httparse::Error> for Error {
        fn from(value: httparse::Error) -> Self {
            Error {
                error_type: ErrorType::HttpParseError,
                message: value.to_string(),
                extra: None
            }
        }
    }

    impl From<serde_json::Error> for Error {
        fn from(value: serde_json::Error) -> Self {
            Error {
                error_type: ErrorType::SerdeError,
                message: value.to_string(),
                extra: None
            }
        }
    }

    impl Error {
        fn as_sendable(&self) -> Result<Vec<u8>> {
            Ok(serde_json::to_vec_pretty(&self)?)
        }

        pub fn missing_path(path: &str) -> Self {
            Error { 
                error_type: ErrorType::MissingPath, 
                message: "Zahtevna pot nima routerja".to_owned(), 
                extra: Some(
                    serde_json::from_str(&format!(r#" {{"path": "{path}"}} "#)).unwrap()
                ) 
            }
        }

        pub fn http_too_long(max_length: &usize) -> Self {
            Error { 
                error_type: ErrorType::HttpRequestTooShort, 
                message: "HTTP request je predolg".to_owned(), 
                extra: Some(
                    serde_json::from_str(&format!(r#" {{"max_length": "{max_length}"}} "#)).unwrap()
                ) 
            }
        }

        // če je mogoče vrne error, drugače samo preskočimo
        pub async fn send_error(self, stream: &mut TcpStream) {
            http::write::write_http("400 Bad Request", &self.as_sendable().unwrap_or_default(), stream).await.unwrap_or_default()
        } 
    }
}

