pub mod settings {
    use std::net::{IpAddr, Ipv4Addr};
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(name = "FloatTorrent ponudnik zaporedij", version, about, long_about=None)]
    pub struct SettingsPonudnik {
        /// IP naslov centralnega strežnika
        #[arg(long)]
        pub centralni_ip:   IpAddr,

        /// Port, na katerem deluje centralni strežnik
        #[arg(long)]
        pub centralni_port: u16,

        /// IP naslov tega ponudnika (se uporablja za registracijo)
        #[arg(short, long, default_value_t=IpAddr::V4(Ipv4Addr::new(0,0,0,0)))]
        pub ip:      IpAddr,

        /// Port tega ponudnika
        #[arg(short, long, default_value_t=9000)]
        pub port:    u16,

        /// Želen čas v sekundah, po katerem se ponudnik znova posvetuje z centralnim in pridobi zaporedja, ki jih ponujajo drugi. 
        #[arg(short, default_value_t=60)]
        pub osvezitveni_cas:  u64
    }

    #[derive(Parser, Debug)]
    #[command(name = "FloatTorrent centralni strežnik generatorjev", version, about, long_about=None)]
    pub struct SettingsCentralni{
        /// IP naslov tega centralnega strežnika
        #[arg(short, long, default_value_t=IpAddr::V4(Ipv4Addr::new(0,0,0,0)))]
        pub ip:      IpAddr,

        /// Port tega ponudnika
        #[arg(short, long, default_value_t=9999)]
        pub port:    u16,

        /// Želen čas v sekundah, po katerem centralni strežnik ping-a vse registrirane, in jih v primeu neodzivnosti odstrani. 
        #[arg(short, default_value_t=60)]
        pub osvezitveni_cas:  u64,

        /// Čas, v sekundah, po katerem se ping izteče
        #[arg(short, default_value_t=5)]
        pub timeout_ping:  u64
    }
}    

pub mod remote {
    use std::net::IpAddr;
    use std::time::Duration;

    use serde::{Deserialize, Serialize};
    use tokio::net::TcpStream;
    use tokio::time::timeout;
    use crate::error::{error::Result, error::Error};
    use crate::http;

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
        pub async fn ping(&self, stream: Option<&mut TcpStream>, timeout_length: u64) -> Result<()> {
            let (reason, status, _) = timeout(Duration::from_secs(timeout_length), self.get("/ping/", stream)).await??;
            if (reason, status) == ("OK".to_owned(), 200) { Ok(()) }
            else {Err(Error::remote_invalid_response("Endpoint /ping/ je vrnil napako"))}
        } 
    }
}

pub mod sequence_provide {        
    use serde::{Deserialize, Serialize};
    use crate::error::{error::Result, error::Error};
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