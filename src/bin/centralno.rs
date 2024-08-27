use std::collections::HashSet;
use std::sync::Arc;

use float_torrent::parse::sequence_provide::Remote;
use float_torrent::{http, error::error::{Error, Result}};
use tokio::{net::{TcpListener, TcpStream}, sync::RwLock};

async fn route_ping (stream: &mut TcpStream, info: &Remote) -> Result<()> {
    http::write::write_http("200 OK", &serde_json::to_vec_pretty(info)?, stream).await?;
    Ok(())
}

async fn route_generator(stream: &mut TcpStream, registered: &Arc<RwLock<HashSet<Remote>>>, data: &[u8]) -> Result<()> {
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
async fn main() -> Result<()>{
    let info = Arc::new(Remote::new("Centralni strežnik", "0.0.0.0", 2222)?);

    let listener = TcpListener::bind(info.get_url()).await?;
    let registered = Arc::new(RwLock::new(HashSet::new()));

    loop {
        let (mut stream, _addr) = listener.accept().await?;
        let registered = registered.clone();
        let info = info.clone();

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
