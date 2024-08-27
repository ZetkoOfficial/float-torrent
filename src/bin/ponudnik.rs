use std::sync::Arc;

use float_torrent::parse::sequence_provide::Remote;
use float_torrent::{parse::sequence_provide, provider::ProviderManager};
use float_torrent::{http, error::error::{Error, Result}};
use tokio::{net::{TcpListener, TcpStream}, sync::RwLock};

// TODO: Erroje handlaj za vse route-e zunaj

async fn route_sequence_generic(path: &str, data: &[u8], stream: &mut TcpStream, manager: &RwLock<ProviderManager>) -> Result<()> {
    let request = sequence_provide::parse_request(&data);
    match request {
        Ok(request) => {
            match manager.read().await.find(&request.get_info(path)) {
                None => Error::missing_provider(&request.get_info(path)).send_error(stream).await,
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

async fn route_sequence(stream: &mut TcpStream, manager: &RwLock<ProviderManager>) -> Result<()> {
    let manager = manager.read().await;

    match serde_json::to_vec_pretty(&manager.get_info()).into() {
        Ok(data) => http::write::write_http("200 OK", &data, stream).await?,
        Err(err) => { let err: Error = err.into(); err.send_error(stream).await }
    };
    Ok(())
}

async fn route_ping (stream: &mut TcpStream, info: &Remote) -> Result<()> {
    match serde_json::to_vec_pretty(info) {
        Ok(data) => http::write::write_http("200 OK", &data, stream).await?,
        Err(err) => { let err: Error = err.into(); err.send_error(stream).await }
    }
    Ok(())
}

async fn register(central_server: &Remote, info: &Remote) -> Result<()> {
    let (reason, status, _) = central_server.post("/generator/", &serde_json::to_vec_pretty(&info)?, None).await?;
    if (reason, status) == ("OK".to_owned(), 200) { Ok(()) } else {Err(Error::remote_invalid_response("Napaka pri registraciji"))}
}

#[tokio::main]
async fn main() -> Result<()>{
    let info = Arc::new(Remote::new("Anže Hočevar", "0.0.0.0", 1111)?);
    let central_server = Arc::new(Remote::new("Centralni strežnik", "0.0.0.0", 2222)?);
    register(&central_server, &info).await?;

    let listener = TcpListener::bind(info.get_url()).await?;
    let manager = Arc::new(RwLock::new(ProviderManager::new()));

    //manager.write().await.register_remote("localhost:2222").await?;

    loop {
        let (mut stream, _addr) = listener.accept().await?;
        let manager = manager.clone();
        let info = info.clone();

        tokio::spawn(async move {
            match http::read::read_http_request(&mut stream).await {
                Err(err) => err.send_error(&mut stream).await,
                Ok((path, data)) => {
                    match path.as_str() {
                        "/sequence/" => route_sequence(&mut stream, &manager).await.unwrap(),
                        "/ping/"     => route_ping(&mut stream, &info).await.unwrap(),
                        path => {
                            if path.starts_with("/sequence/") {
                                match path.get("/sequence/".len()..) {
                                    Some(path) => route_sequence_generic(path, &data, &mut stream, &manager).await.unwrap(),
                                    None => Error::missing_path(path).send_error(&mut stream).await
                                }
                            } else { Error::missing_path(path).send_error(&mut stream).await }
                        } 
                    }
                }
            }
        });

        println!("Ready for new connection...");
    }
}
