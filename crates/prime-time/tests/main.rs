use prime_time::server::{run, Request, Response};
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::test]
async fn is_prime_works() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = format!("localhost:{port}");
    let application = run();
    tokio::spawn(application);
    let request = Request::new("isPrime", 13.0);
    let mut bytes = Vec::new();
    serde_json::to_writer(&mut bytes, &request).unwrap();
    bytes.push(b'\n');
    send_request(&bytes, &address).await;
}

async fn send_request(msg: &[u8], address: &str) {
    let mut stream = TcpStream::connect(address).await.unwrap();
    let result = stream.write_all(msg).await;
    assert!(result.is_ok());

    let mut response = Vec::new();
    stream.read(&mut response).await.unwrap();

    let expected = Response::new("isPrime", true);
    let mut bytes = Vec::new();
    serde_json::to_writer(&mut bytes, &expected).unwrap();
    bytes.push(b'\n');

    assert_eq!(response, bytes);
}
