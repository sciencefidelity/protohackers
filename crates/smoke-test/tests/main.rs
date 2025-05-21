#![allow(clippy::unwrap_used)]
use smoke_test::server::run;
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::test]
async fn smoke_test_works() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = format!("localhost:{port}");
    let application = run();
    tokio::spawn(application);
    tokio::join!(
        send_request(b"hello\n", &address),
        send_request(b"world\n", &address),
        send_request(b"await\n", &address),
        send_request(b"async\n", &address),
        send_request(b"until\n", &address)
    );
}

async fn send_request(msg: &[u8; 6], address: &str) {
    let mut stream = TcpStream::connect(address).await.unwrap();
    let mut response = [0; 6];
    let result = stream.write_all(msg).await;
    assert!(result.is_ok());
    stream.read_exact(&mut response).await.unwrap();
    assert_eq!(&response, msg);
}
