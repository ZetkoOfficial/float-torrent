use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use clap::Parser;
use tokio::{net::{TcpListener, TcpStream}, sync::RwLock};

use common::{
    parse::{settings, remote::Remote},
    error::{Error, Result},
    http,
};

async fn route_ping (stream: &mut TcpStream, info: &Remote) -> Result<()> {
    http::write::write_http("200 OK", &serde_json::to_vec_pretty(info)?, stream).await?;
    Ok(())
}

async fn route_generator(stream: &mut TcpStream, registered: &RwLock<HashSet<Remote>>, data: &[u8]) -> Result<()> {
    // Ločimo med primeroma, ko je body requesta prazen (in želi uporabnik pridobiti registiranje) 
    // in primerom, ko se želi registrirati
    if data.is_empty() {
        let mut result = vec![];
        {
            let registered = registered.read().await;
            for remote in registered.iter() {
                result.push(remote.clone());
            }
        }
        
        http::write::write_http("200 OK", &serde_json::to_vec_pretty(&result)?, stream).await?;
    } else {
        let remote: Remote = serde_json::from_slice(data)?;
        registered.write().await.insert(remote);

        http::write::write_http("200 OK", &[], stream).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = settings::SettingsCentralni::parse();

    let info = Arc::new(Remote::new("Centralni strežnik", &settings.ip.to_string(), settings.port)?);

    let listener = TcpListener::bind(info.get_url()).await?;
    let registered = Arc::new(RwLock::new(HashSet::<Remote>::new()));

    // na vsake toliko časa pingamo vse registriane, če so še aktivni
    { 
        let registered = registered.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(settings.osvezitveni_cas));
            loop {
                interval.tick().await;
                let mut result = HashSet::<Remote>::new();
                for remote in registered.read().await.iter() {
                    if remote.ping(None, settings.timeout_ping).await.is_ok() {
                        result.insert(remote.clone());
                    }
                }
                let mut registered = registered.write().await;
                *registered = result;
                println!("Osveženo!")
            }
        });
    }

    // skrbimo za prihajajoče requeste
    loop {
        let (mut stream, _addr) = listener.accept().await?;
        let info = info.clone();
        let registered = registered.clone();

        tokio::spawn(async move {
            match http::read::read_http_request(&mut stream).await {
                Err(err) => err.send_error(&mut stream).await,
                Ok((path, data)) => {
                    let result = match path.as_str() {
                        "/generator/" => route_generator(&mut stream, &registered, &data).await,
                        "/ping/"     => route_ping(&mut stream, &info).await,
                        _ => { Error::missing_path(&path).send_error(&mut stream).await; Ok(()) }
                    };

                    match result {
                        Ok(()) => (),
                        Err(error) => error.send_error(&mut stream).await
                    }
                }
            }
        });
    }
}
