pub mod read {
    use std::str::from_utf8;

    use tokio::{io::AsyncReadExt, net::TcpStream};

    use crate::error::error::{Error, Result};
    
    const MAX_HTTP_LENGTH: usize = 16384;

    // TODO: Morda povečamo makismalno dolžino requesta kot za respone
    pub async fn read_http_request(stream: &mut TcpStream) -> Result<(String, Vec<u8>)> {
        let mut buffer = [0; MAX_HTTP_LENGTH];  /* 16 kB max */
    
        let mut headers = [httparse::EMPTY_HEADER; 8];
        let mut request = httparse::Request::new(&mut headers);
    
        let read = stream.read(&mut buffer).await?;
        match request.parse(&buffer[..read])? {
            httparse::Status::Partial => Err(Error::http_too_long(&MAX_HTTP_LENGTH)),
            httparse::Status::Complete(offset) => {
                let buffer = &buffer[offset..read];
                match request.path {
                    None => Err(Error::missing_path("NULL")),
                    Some(path) => {
                        Ok((path.to_string(), buffer.to_vec()))
                    }
                }
            }
        }
    }

    pub async fn read_http_response(stream: &mut TcpStream) -> Result<(String, u16, Vec<u8>)> {
        let mut buffer = [0; MAX_HTTP_LENGTH];  /* 16 kB max */
    
        let mut headers = [httparse::EMPTY_HEADER; 8];
        let mut response = httparse::Response::new(&mut headers);
    
        let read = stream.read(&mut buffer).await?;
        match response.parse(&buffer[..read])? {
            httparse::Status::Partial => Err(Error::http_too_long(&MAX_HTTP_LENGTH)),
            httparse::Status::Complete(offset) => {

                let mut data = buffer[offset..read].to_vec();
                let initial_length = data.len();

                let content_length = response.headers.iter()
                    .find(|h| h.name.to_lowercase() == "content-length")
                    .ok_or(Error::malformed_request("Manjka content-length"))?.value;

                let content_length: usize = from_utf8(&content_length)?.parse()?;

                data.resize(content_length, 0);
                stream.read_exact(&mut data[initial_length..]).await?;

                match (response.reason, response.code) {
                    (Some(reason), Some(code)) => Ok((reason.to_owned(), code, data)),
                    _ => Err(Error::http_missing_response())
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

    pub async fn write_post_request(host: &str, endpoint: &str, data: &[u8], stream: &mut TcpStream) -> Result<()> {
        
        let response_start = format!(
            "POST {endpoint} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n", data.len()
        );
        stream.write_all(&response_start.as_bytes()).await?;
        stream.write_all(&data).await?;
        
        Ok(())
    }
    pub async fn write_get_request(host: &str, endpoint: &str, stream: &mut TcpStream) -> Result<()> {

        let response_start = format!(
            "GET {endpoint} HTTP/1.1\r\nHost: {host}\r\n\r\n"
        );
        stream.write_all(&response_start.as_bytes()).await?;        
        Ok(())

    }
}