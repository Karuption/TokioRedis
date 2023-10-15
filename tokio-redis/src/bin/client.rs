use bytes::{Buf, Bytes};
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

enum Command {
    Get {
        key: String,
        response: oneshot::Sender<mini_redis::Result<Option<Bytes>>>,
    },
    Set {
        key: String,
        val: Bytes,
        response: oneshot::Sender<mini_redis::Result<()>>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx1 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();
        tx1.send(Command::Get {
            key: "foo".to_string(),
            response: res_tx,
        })
        .await
        .unwrap();

        println!("Response: {:?}", res_rx.await);
    });

    let tx2 = tx.clone();
    let t2 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            response: res_tx,
        })
        .await
        .unwrap();

        println!("Response: {:?}", res_rx.await);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, response } => {
                    let res = client.get(&key).await;
                    let _ = response.send(res);
                }
                Set { key, val, response } => {
                    let res = client.set(&key, val).await;
                    let _ = response.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
