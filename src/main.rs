use provider::ProviderManager;
use tokio::net::{TcpListener, TcpStream};

mod error; mod parse; mod http; mod provider;
use error::error::{Error, Result};
use parse::sequence_provide::{self};

async fn route_seq_const(data: &[u8], stream: &mut TcpStream, manager: &ProviderManager) -> Result<()> {
    let request = sequence_provide::parse_request(&data);
    match request {
        Ok(request) => { 
            match manager.find(&request.get_info("const")) {
                None => Error::missing_provider(&request.get_info("const")).send_error(stream).await,
                Some(provider) => {
                    match provider.provide(request, manager).await {
                        Err(err) => err.send_error(stream).await,
                        Ok(result) => http::write::write_http("200 OK", &serde_json::to_vec_pretty(&result)?, stream).await? 
                    }
                }
            }
        },
        Err(err) => err.send_error(stream).await
    }
    Ok(())
}

async fn route_sequence(stream: &mut TcpStream, manager: &ProviderManager) -> Result<()> {
    match serde_json::to_vec_pretty(&manager.get_info()).into() {
        Ok(data) => http::write::write_http("200 OK", &data, stream).await?,
        Err(err) => { let err: Error = err.into(); err.send_error(stream).await }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("localhost:1111").await?;
    loop {
        let (mut stream, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let manager = ProviderManager::new();

            match http::read::read_http(&mut stream).await {
                Err(err) => err.send_error(&mut stream).await,
                Ok((path, data)) => {
                    match path.as_str() {
                        "/sequence/const" => route_seq_const(&data, &mut stream, &manager).await.unwrap(),
                        "/sequence/" => route_sequence(&mut stream, &manager).await.unwrap(), 
                        _ => Error::missing_path(&path).send_error(&mut stream).await
                    }
                }
            }
        });

        println!("Ready for new connection...");
    }
}
