pub mod read {
    use tokio::{io::AsyncReadExt, net::TcpStream};

    use crate::error::error::{Error, Result};
    
    pub async fn read_http(stream: &mut TcpStream) -> Result<(String, Vec<u8>)> {
        let mut buffer = [0; 16384];  /* 16 kB max */
    
        let mut headers = [httparse::EMPTY_HEADER; 8];
        let mut request = httparse::Request::new(&mut headers);
    
        let read = stream.read(&mut buffer).await?;
        match request.parse(&buffer[..read])? {
            httparse::Status::Partial => Err(Error::HttpRequestTooShort("HTTP request is too long".into())),
            httparse::Status::Complete(offset) => {
                let buffer = &buffer[offset..read];
                match request.path {
                    None => Err(Error::MissingPath("Specified path has no router".into())),
                    Some(path) => {
                        Ok((path.to_string(), buffer.to_vec()))
                    }
                }
            }
        }
    }
}

pub mod write {
    use tokio::{io::AsyncWriteExt, net::TcpStream};

    use crate::error::error::Result;

    pub async fn write_http(status: &str, data: &[u8], stream: &mut TcpStream) -> Result<()> {

        let response_start = format!("HTTP/1.1 {status}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n", data.len());
        stream.write_all(&response_start.as_bytes()).await?;
        stream.write_all(&data).await?;

        Ok(())
    }
}