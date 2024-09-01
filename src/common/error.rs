/// Tukaj definiramo tip Result in Error, ki ga uporabljamo povsod, in 
/// zapišemo pretvorbe med tipi Error

use std::{io, net::AddrParseError, num::{ParseIntError, TryFromIntError}, result, str::Utf8Error};

use serde::Serialize;
use serde_json::json;
use tokio::{net::TcpStream, time::error::Elapsed};

use crate::{http, parse::{parse_helper::Sendable, sequence_provide}};

#[derive(Debug, Serialize, PartialEq)]
#[allow(dead_code)]
pub enum ErrorType {
    IOError,
    JSONParseError,
    GenericParseError,
    HttpParseError,
    HttpRequestTooShort,
    MissingPath,
    MissingProvider,
    RemoteError,
    Timeout,
    ArithmeticError,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Error {
    error:      ErrorType,
    message:    String,
    extra:      Option<serde_json::Value>
}

pub type Result<T> = result::Result<T, Error>;
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error {
            error: ErrorType::IOError,
            message: value.to_string(),
            extra: None
        }
    }
}
impl From<httparse::Error> for Error {
    fn from(value: httparse::Error) -> Self {
        Error {
            error: ErrorType::HttpParseError,
            message: value.to_string(),
            extra: None
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error {
            error: ErrorType::JSONParseError,
            message: value.to_string(),
            extra: None
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Error {
            error: ErrorType::HttpParseError,
            message: value.to_string(),
            extra: None
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error {
            error: ErrorType::GenericParseError,
            message: value.to_string(),
            extra: None
        }
    }
}

impl From<AddrParseError> for Error {
    fn from(value: AddrParseError) -> Self {
        Error {
            error: ErrorType::GenericParseError,
            message: value.to_string(),
            extra: None
        }
    }
}

impl From<Elapsed> for Error {
    fn from(value: Elapsed) -> Self {
        Error {
            error: ErrorType::Timeout,
            message: value.to_string(),
            extra: None
        }
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error {
            error: ErrorType::ArithmeticError,
            message: value.to_string(),
            extra: None
        }
    }
}

impl Sendable for Error {}
impl Error {
    pub fn missing_path(path: &str) -> Self {
        Error { 
            error: ErrorType::MissingPath, 
            message: "Zahtevna pot nima routerja".to_owned(), 
            extra: Some(
                serde_json::from_str(&format!(r#" {{"path": "{path}"}} "#)).unwrap()
            ) 
        }
    }

    pub fn http_too_long(max_length: &usize) -> Self {
        Error { 
            error: ErrorType::HttpRequestTooShort, 
            message: "HTTP request je predolg".to_owned(), 
            extra: Some(
                serde_json::from_str(&format!(r#" {{"max_length": "{max_length}"}} "#)).unwrap()
            ) 
        }
    }

    pub fn missing_provider(seq: sequence_provide::SequenceInfo, close: &[sequence_provide::SequenceInfo]) -> Self {
        Error { 
            error: ErrorType::MissingProvider, 
            message: "Ponudnik zaporedja ni najden. Blizu so so spodnji ponudniki.".to_owned(), 
            extra: Some(
                json!({
                    "_query":   serde_json::to_value(seq).unwrap(),
                    "close":    serde_json::to_value(close).unwrap()  
                })
            ) 
        }
    }

    pub fn sequence_arithmetic_error(seq: sequence_provide::SequenceInfo, extra: &str) -> Self {
        Error { 
            error: ErrorType::ArithmeticError, 
            message: "Napaka pri računanju členov".to_owned(), 
            extra: Some(
                json!({
                    "_sequence": seq,
                    "info": extra 
                })
            ) 
        }
    }

    pub fn http_missing_response() -> Self {
        Error { 
            error: ErrorType::HttpParseError, 
            message: "Manjkajoč reason/code pri response-u.".to_owned(), 
            extra: None 
        }
    }
    
    pub fn invalid_range() -> Self {
        Error {
            error: ErrorType::GenericParseError,
            message: "Neveljaven range".to_owned(),
            extra: None
        }
    }

    pub fn malformed_request(extra: &str) -> Self {
        Error {
            error: ErrorType::HttpParseError,
            message: "HTTP zahteva ni veljavne oblike.".to_owned(),
            extra: Some(serde_json::Value::String(extra.to_owned()))
        }
    }

    pub fn remote_invalid_response(url: &str, error: &[u8]) -> Self {
        let error_json: Option<serde_json::Value> = serde_json::from_slice(error).ok();
        Error {
            error: ErrorType::RemoteError,
            message: "Remote se je odzval narobe.".to_owned(),
            extra: Some(json!({
                "_url":      serde_json::Value::String(url.to_owned()),
                "info":    error_json
            }))
        }
    }

    // če je mogoče vrne error, drugače samo preskočimo
    pub async fn send_error(self, stream: &mut TcpStream) {
        http::write::write_http("400 Bad Request", &self.as_sendable().unwrap_or_default(), stream).await.unwrap_or_default()
    } 
}

