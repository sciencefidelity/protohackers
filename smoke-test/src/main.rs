use log::{debug, error, info};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    info!("starting up");
    let port = std::env::var("TCP_PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = format!("0.0.0.0:{port}");
    serve(&address)?;
    Ok(())
}

/// Listen for TCP connections on the specified socket address.
fn serve(address: &str) -> Result<(), failure::Error> {
    info!("listening on {address}");
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || handler(stream).unwrap_or_else(|error| error!("{error:?}")));
    }
}

/// Wait for data(message) from the client and return the same content back.
fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        stream.write_all(&buffer[..nbytes])?;
    }
}
