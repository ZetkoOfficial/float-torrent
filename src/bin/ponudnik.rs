use std::sync::Arc;

use float_torrent::parse::sequence_provide::Remote;
use float_torrent::{parse::sequence_provide, provider::ProviderManager};
use float_torrent::{http, error::error::{Error, Result}};
use tokio::{net::{TcpListener, TcpStream}, sync::RwLock};

async fn route_sequence_generic(path: &str, data: &[u8], stream: &mut TcpStream, manager: &RwLock<ProviderManager>) -> Result<()> {
    let request = sequence_provide::parse_request(&data)?;
    
    match manager.read().await.find(&request.get_info(path)) {
        None => Err(Error::missing_provider(&request.get_info(path))),
        Some(provider) => {
            let result = provider.provide(request, manager).await?;
            http::write::write_http("200 OK", &serde_json::to_vec_pretty(&result)?, stream).await
        }
    }
}

async fn route_sequence(stream: &mut TcpStream, manager: &RwLock<ProviderManager>) -> Result<()> {
    let data: Vec<u8> = serde_json::to_vec_pretty(&manager.read().await.get_info())?;
    http::write::write_http("200 OK", &data, stream).await
}

async fn route_ping (stream: &mut TcpStream, info: &Remote) -> Result<()> {
    let data = serde_json::to_vec_pretty(info)?;
    http::write::write_http("200 OK", &data, stream).await
}

async fn register(central_server: &Remote, info: &Remote) -> Result<()> {
    let (reason, status, _) = central_server.post("/generator/", &serde_json::to_vec_pretty(&info)?, None).await?;
    if (reason, status) == ("OK".to_owned(), 200) { Ok(()) } else { Err(Error::remote_invalid_response("Napaka pri registraciji")) }
}

#[tokio::main]
async fn main() -> Result<()>{
    let info = Arc::new(Remote::new("Anže Hočevar", "0.0.0.0", 1111)?);
    let central_server = Arc::new(Remote::new("Centralni strežnik", "0.0.0.0", 2222)?);
    register(&central_server, &info).await?;

    let listener = TcpListener::bind(info.get_url()).await?;
    let manager = Arc::new(RwLock::new(ProviderManager::new(&info, &central_server)));
    
    // TODO: To naj bo task, ki se izvede vsakih nekaj minut, da se providerji osvežijo
    ProviderManager::update_providers(&manager).await?;

    loop {
        let (mut stream, _addr) = listener.accept().await?;
        let manager = manager.clone();
        let info = info.clone();

        tokio::spawn(async move {
            match http::read::read_http_request(&mut stream).await {
                Err(err) => err.send_error(&mut stream).await,
                Ok((path, data)) => {
                    let result = match path.as_str() {                        
                        "/sequence/"    => route_sequence(&mut stream, &manager).await,
                        "/ping/"        => route_ping(&mut stream, &info).await,
                        path      => {
                            if path.starts_with("/sequence/") {
                                match path.get("/sequence/".len()..) {
                                    Some(path) => route_sequence_generic(path, &data, &mut stream, &manager).await,
                                    None => Err(Error::missing_path(path)),
                                }
                            } else { Err(Error::missing_path(path)) }
                        } 
                    };
                    match result {
                        Ok(()) => (),
                        Err(err) => err.send_error(&mut stream).await
                    }
                }
            }
        });

        println!("Ready for new connection...");
    }
}
