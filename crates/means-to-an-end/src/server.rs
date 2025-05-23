use std::{collections::BTreeMap, net::SocketAddr, str};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{event, instrument, Level};

#[derive(Debug)]
enum Message {
    Insert,
    Query,
}

#[instrument]
pub async fn run() -> std::io::Result<()> {
    event!(Level::INFO, "starting up");
    let port = std::env::var("TCP_PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = format!("0.0.0.0:{port}");
    serve(&address).await?;
    Ok(())
}

#[instrument]
async fn serve(address: &str) -> std::io::Result<()> {
    event!(Level::INFO, "listening on {address}");
    let listener = TcpListener::bind(address).await?;
    loop {
        let (socket, src) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket, &src)
                .await
                .unwrap_or_else(|error| event!(Level::ERROR, "{error:?}"));
        });
    }
}

#[instrument]
async fn process(mut stream: TcpStream, src: &SocketAddr) -> std::io::Result<()> {
    event!(Level::DEBUG, "handling data from {}", stream.peer_addr()?);
    let mut storage = BTreeMap::new();
    loop {
        let mut header = [0; 1];
        stream.read_exact(&mut header).await?;
        let msg = match header[0] as char {
            'I' => Message::Insert,
            'Q' => Message::Query,
            sym => {
                event!(Level::ERROR, "Unknow message header {sym}");
                break;
            }
        };
        let mut op_1 = [0; 4];
        stream.read_exact(&mut op_1).await?;
        let op_1 = i32::from_be_bytes(op_1);
        let mut op_2 = [0; 4];
        stream.read_exact(&mut op_2).await?;
        let op_2 = i32::from_be_bytes(op_2);
        match msg {
            Message::Insert => {
                event!(Level::DEBUG, "insert {op_1}: {op_2}");
                storage.insert(op_1, op_2);
            }
            Message::Query => {
                if op_1 > op_2 {
                    stream.write_all(&0i32.to_be_bytes()).await?;
                    stream.flush().await?;
                    continue;
                }
                let (mut count, mut sum) = (0, 0);
                for (_, item) in storage.range(op_1..=op_2) {
                    count += 1;
                    sum += *item as i128;
                }
                let mean = if count == 0 { 0 } else { sum / count };
                event!(Level::DEBUG, "query {mean}");
                stream.write(&(mean as i32).to_be_bytes()).await?;
                stream.flush().await?;
            }
        }
    }

    event!(Level::INFO, "shutting down stream {src}");
    stream.shutdown().await?;
    Ok(())
}
