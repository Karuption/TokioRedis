use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("unable to start TCP listener");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { process(socket).await });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};

    let mut map = std::collections::HashMap::new();

    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let command = Command::from_frame(frame).unwrap();
        println!("Received command: {:?}", command);

        let res = match command {
            Set(cmd) => {
                map.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }

            Get(cmd) => {
                if let Some(value) = map.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("not implemented {:?}", cmd),
        };

        conn.write_frame(&res).await.unwrap();
    }
}
