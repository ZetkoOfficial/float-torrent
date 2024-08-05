pub mod get {
    use std::{io::Read, net::TcpStream};
    use crate::error::error::{Error,Result};
    
    pub fn get_http(stream: &mut TcpStream) -> Result<(String, Vec<u8>)> {
        let mut buffer = [0; 16384];  /* 16 kB max */
    
        let mut headers = [httparse::EMPTY_HEADER; 8];
        let mut request = httparse::Request::new(&mut headers);
    
        let read = stream.read(&mut buffer)?;
        match request.parse(&buffer[..read])? {
            httparse::Status::Partial => Err(Error::HttpRequestTooShort),
            httparse::Status::Complete(offset) => {
                let buffer = &buffer[offset..read];
                match request.path {
                    None => Err(Error::MissingPath),
                    Some(path) => {
                        Ok((path.to_string(), buffer.to_vec()))
                    }
                }
            }
        }
    }
}

