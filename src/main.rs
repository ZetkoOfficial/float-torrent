use tokio::net::{TcpListener, TcpStream};

mod error; mod parse; mod http;
use error::error::{Error, ErrorResponse, Result};
use parse::sequence_provide;

async fn route_example_provide(data: &[u8], stream: &mut TcpStream) -> Result<()> {
    let request = sequence_provide::parse_request(&data);
    match request {
        Ok(req) => { http::write::write_http("200 OK", &[], stream).await?; println!("{:?}", req) },
        Err(err) => err.send_error(stream).await
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("localhost:1111").await?;
    loop {
        let (mut stream, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            match http::read::read_http(&mut stream).await {
                Err(err) => err.send_error(&mut stream).await,
                Ok((path, data)) => {
                    match path.as_str() {
                        "/example_provide" => route_example_provide(&data, &mut stream).await.unwrap(),
                        _ => Error::MissingPath(ErrorResponse::new_with_extra(
                            "Zahtevana pot nima routerja.",
                            &format!(r#"{{"path": "{}"}}"#, path)
                        )).send_error(&mut stream).await
                    }
                }
            }
        });

        println!("Ready for new connection...");
    }
}
