use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379")
        .await
        .expect("Cannot Connect to Server");

    client
        .set("hello", "world".into())
        .await
        .expect("Unable to Set KVP");

    let result = client.get("hello").await.expect("Unable to get the result");
    println!("Got value from the server, {:?}", result);

    Ok(())
}
