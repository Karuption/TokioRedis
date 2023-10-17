use std::fs::read;
use std::net::SocketAddr;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let mut server = TcpListener::bind("127.0.0.1:23").await.unwrap();

    loop {
        let (mut socket, _) = server.accept().await.unwrap();

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            io::copy(&mut reader, &mut writer).await.unwrap();
        });
    }
}
