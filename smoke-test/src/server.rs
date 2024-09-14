use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{event, instrument, Level};

#[instrument]
pub async fn run() -> Result<(), failure::Error> {
    event!(Level::INFO, "starting up");
    let port = std::env::var("TCP_PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = format!("0.0.0.0:{port}");
    serve(&address).await?;
    Ok(())
}

/// Listen for TCP connections on the specified socket address.
#[instrument]
async fn serve(address: &str) -> Result<(), failure::Error> {
    event!(Level::INFO, "listening on {address}");
    let listener = TcpListener::bind(address).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket)
                .await
                .unwrap_or_else(|error| event!(Level::ERROR, "{error:?}"));
        });
    }
}

/// Wait for data(message) from the client and return the same content back.
#[instrument]
async fn process(mut stream: TcpStream) -> Result<(), failure::Error> {
    event!(Level::DEBUG, "handling data from {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        let nbytes = stream.read(&mut buffer).await?;
        if nbytes == 0 {
            event!(Level::DEBUG, "connection closed");
            return Ok(());
        }
        stream.write_all(&buffer[..nbytes]).await?;
    }
}
