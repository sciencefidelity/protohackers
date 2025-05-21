use crate::integer::Integer;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{event, instrument, Level};

pub const METHOD: &str = "isPrime";

#[derive(Deserialize, Serialize, Debug)]
pub struct Request<'a> {
    method: &'a str,
    number: f64,
}

impl<'a> Request<'a> {
    pub fn new(method: &'a str, number: f64) -> Self {
        Self { method, number }
    }
}

#[derive(Serialize, Debug)]
pub struct Response<'a> {
    method: &'a str,
    prime: bool,
}

impl<'a> Response<'a> {
    pub fn new(method: &'a str, prime: bool) -> Self {
        Self { method, prime }
    }
}

#[instrument]
pub async fn run() -> anyhow::Result<()> {
    event!(Level::INFO, "starting up");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = format!("0.0.0.0:{port}");
    serve(&address).await?;
    Ok(())
}

#[instrument]
async fn serve(address: &str) -> anyhow::Result<()> {
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

#[instrument]
async fn process(mut stream: TcpStream) -> anyhow::Result<()> {
    event!(Level::DEBUG, "handling data from {}", stream.peer_addr()?);
    let (reader, mut writer) = stream.split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        let response: Response = match serde_json::from_str::<Request>(&line) {
            Ok(request) if request.method == METHOD => {
                Response::new(METHOD, request.number.is_prime())
            }
            _ => Response::new("MALFORMED", false),
        };
        event!(Level::DEBUG, "response: {response:?}");
        let mut bytes = Vec::new();
        serde_json::to_writer(&mut bytes, &response)?;
        bytes.push(b'\n');
        writer.write_all(&bytes).await?;
    }
    Ok(())
}
