pub mod error {
    use std::{io, net::AddrParseError, num::{ParseIntError, TryFromIntError}, result, str::Utf8Error};

    use serde::Serialize;
    use serde_json::json;
    use tokio::{net::TcpStream, time::error::Elapsed};

    use crate::{http, parse::{parse_helper::Sendable, sequence_provide}};

    #[derive(Debug, Serialize)]
    #[allow(dead_code)]
    pub enum ErrorType {
        IOError,
        GenericParseError,
        HttpParseError,
        SerdeError,
        HttpRequestTooShort,
        MissingPath,
        MissingProvider,
        SequenceArithmeticError,
        RemoteError,
        Timeout,
        ArithmeticError,
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

    impl From<Utf8Error> for Error {
        fn from(value: Utf8Error) -> Self {
            Error {
                error_type: ErrorType::HttpParseError,
                message: value.to_string(),
                extra: None
            }
        }
    }

    impl From<ParseIntError> for Error {
        fn from(value: ParseIntError) -> Self {
            Error {
                error_type: ErrorType::GenericParseError,
                message: value.to_string(),
                extra: None
            }
        }
    }

    impl From<AddrParseError> for Error {
        fn from(value: AddrParseError) -> Self {
            Error {
                error_type: ErrorType::GenericParseError,
                message: value.to_string(),
                extra: None
            }
        }
    }

    impl From<Elapsed> for Error {
        fn from(value: Elapsed) -> Self {
            Error {
                error_type: ErrorType::Timeout,
                message: value.to_string(),
                extra: None
            }
        }
    }

    impl From<TryFromIntError> for Error {
        fn from(value: TryFromIntError) -> Self {
            Error {
                error_type: ErrorType::ArithmeticError,
                message: value.to_string(),
                extra: None
            }
        }
    }

    impl Sendable for Error {}
    impl Error {
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

        pub fn missing_provider(seq: sequence_provide::SequenceInfo, close: &[sequence_provide::SequenceInfo]) -> Self {
            Error { 
                error_type: ErrorType::MissingProvider, 
                message: "Ponudnik zaporedja ni najden. Blizu so so spodnji ponudniki.".to_owned(), 
                extra: Some(
                    json!({
                        "_query":   serde_json::to_value(seq).unwrap(),
                        "close":    serde_json::to_value(close).unwrap()  
                    })
                ) 
            }
        }

        pub fn sequence_arithmetic_error(extra: &str) -> Self {
            Error { 
                error_type: ErrorType::SequenceArithmeticError, 
                message: "Napaka pri računanju členov".to_owned(), 
                extra: Some(
                    serde_json::Value::String(extra.to_owned())
                ) 
            }
        }

        pub fn http_missing_response() -> Self {
            Error { 
                error_type: ErrorType::HttpParseError, 
                message: "Manjkajoč reason/code pri response-u.".to_owned(), 
                extra: None 
            }
        }
        
        pub fn invalid_range() -> Self {
            Error {
                error_type: ErrorType::SerdeError,
                message: "Neveljaven range".to_owned(),
                extra: None
            }
        }

        pub fn malformed_request(extra: &str) -> Self {
            Error {
                error_type: ErrorType::HttpParseError,
                message: "HTTP zahteva ni veljavne oblike.".to_owned(),
                extra: Some(serde_json::Value::String(extra.to_owned()))
            }
        }

        pub fn remote_invalid_response(extra: &str) -> Self {
            Error {
                error_type: ErrorType::RemoteError,
                message: "Remote se je odzval narobe.".to_owned(),
                extra: Some(serde_json::Value::String(extra.to_owned()))
            }
        }

        // če je mogoče vrne error, drugače samo preskočimo
        pub async fn send_error(self, stream: &mut TcpStream) {
            http::write::write_http("400 Bad Request", &self.as_sendable().unwrap_or_default(), stream).await.unwrap_or_default()
        } 
    }
}

