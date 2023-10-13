use mini_redis::client::Client;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = get_session().await;

    client
        .set("hello", "world".into())
        .await
        .expect("Unable to Set KVP");

    drop(client);
    for _ in 0..5 {
        let mut client = get_session().await;

        let result = client.get("hello").await.expect("Unable to get the result");
        println!("Got value from the server, {:?}", result);
    }

    Ok(())
}

async fn get_session() -> Client {
    client::connect("127.0.0.1:6379")
        .await
        .expect("Cannot Connect to Server")
}
