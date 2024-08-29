pub mod sequence_provide {
    use std::net::IpAddr;

    use serde::{Deserialize, Serialize};
    use tokio::net::TcpStream;
    use crate::error::{error::Result, error::Error};
    use crate::http;

    use super::parse_helper::Sendable;

    #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
    pub struct Range {
        pub from:   u64,
        pub to:     u64,
        pub step:   u64
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SequenceParameter {
        pub name: String, 
        pub parameters: Vec<f64>,
        pub sequences: Vec<SequenceParameter>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SequenceInfo {
        pub name: String,
        pub description: String,
        pub parameters: usize,
        pub sequences: usize
    }
    impl Sendable for SequenceInfo {}
    impl PartialEq for SequenceInfo {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name && self.parameters == other.parameters && self.sequences == other.sequences
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub range: Range,
        pub parameters: Vec<f64>,
        pub sequences: Vec<SequenceParameter>
    }
    impl Sendable for Request {}

    #[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
    pub struct Remote {
        pub name: String,
        pub ip  : IpAddr,
        pub port: u16
    }
    impl Remote {
        pub fn get_url(&self) -> String {
            format!("{}:{}", self.ip, self.port)
        }

        pub fn new(name: &str, ip: &str, port: u16) -> Result<Self> {
            let ip : IpAddr = ip.parse()?;

            Ok(Remote {
                name: name.to_owned(),
                ip,
                port
            })
        }

        pub async fn get_stream(&self) -> Result<TcpStream> {
            Ok(TcpStream::connect(&self.get_url()).await?)
        }
        pub async fn get(&self, endpoint: &str, stream: Option<&mut TcpStream>) -> Result<(String, u16, Vec<u8>)> {
            let stream: &mut TcpStream = match stream {
                None => &mut (self.get_stream().await?),
                Some(stream) => stream
            };
            http::write::write_get_request(&self.get_url(), endpoint, stream).await?;
            http::read::read_http_response(stream).await
        }
        pub async fn post(&self, endpoint: &str, data: &[u8], stream: Option<&mut TcpStream>) -> Result<(String, u16, Vec<u8>)> {
            let stream: &mut TcpStream = match stream {
                None => &mut (self.get_stream().await?),
                Some(stream) => stream
            };
            http::write::write_post_request(&self.get_url(), endpoint, data, stream).await?;
            http::read::read_http_response(stream).await
        }
    }

    impl Request {
        pub fn validate(self) -> Result<Self> {
            if self.range.from <= self.range.to && self.range.step > 0 {
                Ok(self)
            } else { Err(Error::invalid_range()) }
        }

        pub fn get_info(&self, name: &str) -> SequenceInfo {
            SequenceInfo {
                name: name.to_owned(),
                parameters: self.parameters.len(),
                sequences: self.sequences.len(),
                description: "".to_owned()
            }
        }
    }

    impl SequenceParameter {
        pub fn get_info(&self) -> SequenceInfo {
            SequenceInfo {
                name: self.name.to_owned(),
                parameters: self.parameters.len(),
                sequences: self.sequences.len(),
                description: "".to_owned()
            }
        } 
    }

    pub fn parse_request(data: &[u8]) -> Result<Request> { 
        let request: Request = serde_json::from_slice(&data)?;
        request.validate()
    }
}

pub mod parse_helper {
    use crate::error::error::Result;

    pub trait Sendable : serde::Serialize {
        fn as_sendable(&self) -> Result<Vec<u8>> {
            Ok(serde_json::to_vec_pretty(&self)?)
        }
    }
}