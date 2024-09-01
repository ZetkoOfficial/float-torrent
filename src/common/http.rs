pub mod read {
    use std::str::from_utf8;
    use tokio::{io::AsyncReadExt, net::TcpStream};
    use crate::error::{Error, Result};
    
    const BUFFER_LENGTH: usize = 16384;

    /// Preberemo HTTP request, tudi če je dolg in vrnemo (path, body)
    pub async fn read_http_request(stream: &mut TcpStream) -> Result<(String, Vec<u8>)> {
        let mut buffer = vec![];
        
        loop {
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut request = httparse::Request::new(&mut headers);

            {
                let mut buffer_tmp = [0; BUFFER_LENGTH];
                let read_current = stream.read(&mut buffer_tmp).await?;

                buffer.extend_from_slice(&buffer_tmp[..read_current]);
            }
            
            match request.parse(&buffer)? {
                httparse::Status::Partial => continue,
                httparse::Status::Complete(_) => break
            }
        };

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut request = httparse::Request::new(&mut headers);
            
        let mut remaining = match request.parse(&buffer)? {
            httparse::Status::Partial => Err(Error::http_too_long(&BUFFER_LENGTH))?,
            httparse::Status::Complete(offset) => buffer[offset..].to_vec()
        };

        // preverimo če ima body
        let content_length: usize = 
            match request.headers.iter()
            .find(|h| h.name.to_lowercase() == "content-length")
            .ok_or(Error::malformed_request("Manjka content-length")) {
                Ok(header) => from_utf8(&header.value)?.parse()?,
                Err(_) => 0 
        };

        // pridobimo preostanek
        let pre_len = remaining.len();
        remaining.resize(content_length, 0);
        if pre_len < content_length {
            stream.read_exact(&mut remaining[pre_len..]).await?;
        }

        match request.path {
            None => Err(Error::missing_path("NULL")),
            Some(path) => {
                Ok((path.to_string(), remaining.clone()))
            }
        }
    }

    /// Preberemo HTTP response, tudi če je dolg in vrnemo (reason, code, body)
    pub async fn read_http_response(stream: &mut TcpStream) -> Result<(String, u16, Vec<u8>)> {

        let mut buffer = vec![];
        
        loop {
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut response = httparse::Response::new(&mut headers);

            {
                let mut buffer_tmp = [0; BUFFER_LENGTH];
                let read_current = stream.read(&mut buffer_tmp).await?;

                buffer.extend_from_slice(&buffer_tmp[..read_current]);
            }
            
            match response.parse(&buffer)? {
                httparse::Status::Partial => continue,
                httparse::Status::Complete(_) => break
            }
        };

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut response = httparse::Response::new(&mut headers);
            
        let mut remaining = match response.parse(&buffer)? {
            httparse::Status::Partial => Err(Error::http_too_long(&BUFFER_LENGTH))?,
            httparse::Status::Complete(offset) => buffer[offset..].to_vec()
        };

        // preverimo če ima body
        let content_length: usize = 
            match response.headers.iter()
            .find(|h| h.name.to_lowercase() == "content-length")
            .ok_or(Error::malformed_request("Manjka content-length")) {
                Ok(header) => from_utf8(&header.value)?.parse()?,
                Err(_) => 0 
        };

        // pridobimo preostanek
        let pre_len = remaining.len();
        remaining.resize(content_length, 0);
        if pre_len < content_length {
            stream.read_exact(&mut remaining[pre_len..]).await?;
        }
    
        match (response.reason, response.code) {
            (Some(reason), Some(code)) => Ok((reason.to_owned(), code, remaining)),
            _ => Err(Error::http_missing_response())
        }
    }
}

pub mod write {
    use tokio::{io::AsyncWriteExt, net::TcpStream};
    use crate::error::Result;

    /// Pošljemo HTTP response
    pub async fn write_http(status: &str, data: &[u8], stream: &mut TcpStream) -> Result<()> {

        let response_start = format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n", data.len());
        stream.write_all(&response_start.as_bytes()).await?;
        stream.write_all(&data).await?;

        Ok(())
    }

    /// Pošljemo HTTP POST request
    pub async fn write_post_request(host: &str, endpoint: &str, data: &[u8], stream: &mut TcpStream) -> Result<()> {
        
        let response_start = format!(
            "POST {endpoint} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n", data.len()
        );
        stream.write_all(&response_start.as_bytes()).await?;
        stream.write_all(&data).await?;
        
        Ok(())
    }

    /// Pošljemo HTTP GET request
    pub async fn write_get_request(host: &str, endpoint: &str, stream: &mut TcpStream) -> Result<()> {

        let response_start = format!(
            "GET {endpoint} HTTP/1.1\r\nHost: {host}\r\n\r\n"
        );
        stream.write_all(&response_start.as_bytes()).await?;        
        Ok(())

    }
}