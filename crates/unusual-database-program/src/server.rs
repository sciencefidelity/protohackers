use std::collections::HashMap;

use tokio::net::UdpSocket;
use tracing::{event, Level};

const VERSION: &str = "Ken's Key-Value Store 1.0";

pub async fn run() -> anyhow::Result<()> {
    event!(Level::INFO, "starting up");
    let port = std::env::var("UDP_PORT").unwrap_or_else(|_| "51820".to_owned());
    let address = format!("0.0.0.0:{port}");
    serve(&address).await?;
    Ok(())
}

pub async fn serve(address: &str) -> anyhow::Result<()> {
    let mut database = HashMap::new();
    event!(Level::INFO, "listening on {address}");
    let socket = UdpSocket::bind(address).await?;

    let mut buf = [0; 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf).await?;
        let buf = &mut buf[..amt];
        let command = str::from_utf8(buf)?;

        event!(Level::DEBUG, "handling data from {src}");
        event!(Level::DEBUG, "COMMAND: {command}");

        if command.starts_with("version") {
            socket
                .send_to(format!("version={VERSION}").as_bytes(), &src)
                .await?;
            continue;
        }
        if command.contains("=") {
            let i = command.find("=").expect("insert missing equals");
            database.insert(command[..i].to_string(), command[i + 1..].to_string());
            continue;
        }
        let value = database.get(command).map_or("", |val| val);
        socket
            .send_to(format!("{command}={value}").as_bytes(), &src)
            .await?;
    }
}
