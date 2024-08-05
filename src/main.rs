use std::net::{TcpListener, TcpStream};

mod error; mod parse; mod http;
use error::error::Result;
use parse::sequence_provide;

fn parse_sequence_provide_request(stream: &mut TcpStream) -> Result<sequence_provide::Request> {
    let (_path, data) = http::get::get_http(stream)?;
    sequence_provide::parse_request(&data)
}

fn main() -> Result<()>{
    let listener = TcpListener::bind("localhost:1111")?;
    loop {
        let mut stream = listener.accept()?;
        let data =parse_sequence_provide_request(&mut stream.0);
        println!("{:?}", data)
    }
}
