use std::sync::Arc;
use std::time::Duration;
use clap::Parser;
use tokio::net::TcpListener;
use tokio::{net::TcpStream, sync::RwLock};

use common::{
    parse::{remote::Remote, sequence_provide, settings}, 
    sequnce_providers::ProviderManager, 
    error::{Error, Result}, 
    http
};

// če je mogoče vrne generirano zaporedje z iskano signaturo
async fn route_sequence_generic(path: &str, data: &[u8], stream: &mut TcpStream, manager: &RwLock<ProviderManager>) -> Result<()> {
    let request = sequence_provide::parse_request(&data)?;
    
    let result = manager.read().await
        .find(&request.get_info(path))?
        .provide(request, manager).await?;

    http::write::write_http("200 OK", &serde_json::to_vec_pretty(&result)?, stream).await
}

// vrne seznam zaporedij
async fn route_sequence(stream: &mut TcpStream, manager: &RwLock<ProviderManager>) -> Result<()> {
    let data: Vec<u8> = serde_json::to_vec_pretty(&manager.read().await.get_info())?;
    http::write::write_http("200 OK", &data, stream).await
}

async fn route_ping (stream: &mut TcpStream, info: &Remote) -> Result<()> {
    let data = serde_json::to_vec_pretty(info)?;
    http::write::write_http("200 OK", &data, stream).await
}

// registrira sebe na endpoint /generator/, centralnega strežnika 
async fn register(register_endpoint: &str, central_server: &Remote, info: &Remote) -> Result<()> {
    let (reason, status, data) = central_server.post(register_endpoint, &serde_json::to_vec_pretty(&info)?, None).await?;
    if (reason, status) == ("OK".to_owned(), 200) { Ok(()) } else { Err(Error::remote_invalid_response(&central_server.get_url(), &data)) }
}

#[tokio::main]
async fn main() -> Result<()> {

    let settings = settings::SettingsPonudnik::parse();
    let register_endpoint = http::helper::remove_if_trailing(&settings.register_endpoint);

    let info = Arc::new(Remote::new("Anže Hočevar", &settings.ip.to_string(), settings.port)?);
    let central_server = Arc::new(Remote::new("Centralni strežnik", &settings.centralni_ip.to_string(), settings.centralni_port)?);
    register(&register_endpoint, &central_server, &info).await?;

    let listener = TcpListener::bind(info.get_url()).await?;
    let manager = Arc::new(RwLock::new(ProviderManager::new(&settings, &info, &central_server)));

    // na vsake tokliko časa posobimo naše remote ponudnike
    { 
        let manager = manager.clone();
        let register_endpoint = register_endpoint.to_string();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(settings.osvezitveni_cas));
            loop {
                interval.tick().await;
                ProviderManager::update_providers(&register_endpoint, &manager).await.unwrap_or_else(|_| println!("Napaka pri posodabljanju remote providerjev."));
            }
        });
    }

    // sprejemamo requeste tukaj
    loop {
        let (mut stream, _addr) = listener.accept().await?;
        let manager = manager.clone();
        let info = info.clone();

        tokio::spawn(async move {
            match http::read::read_http_request(&mut stream).await {
                Err(err) => err.send_error(&mut stream).await,
                Ok((path, data)) => {
                    let result = match http::helper::remove_if_trailing(path.as_str()) {                        
                        "/sequence"    => route_sequence(&mut stream, &manager).await,
                        "/ping"        => route_ping(&mut stream, &info).await,
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
    }
}
