use bytes::{Buf, Bytes};
use mini_redis::client;
use tokio::sync::mpsc;

enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx1 = tx.clone();
    let t1 = tokio::spawn(async move {
        tx1.send(Command::Get {
            key: "foo".to_string(),
        })
        .await
        .unwrap()
    });

    let tx2 = tx.clone();
    let t2 = tokio::spawn(async move {
        tx2.send(Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        })
        .await
        .unwrap()
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key } => {
                    client.get(&key).await.unwrap();
                }
                Set { key, val } => {
                    client.set(&key, val).await.unwrap();
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
