use mini_redis::client::Client;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    {
        get_session()
            .await
            .set("hello", "world".into())
            .await
            .expect("Unable to Set KVP");
    }

    for _ in 0..5 {
        let result = get_session()
            .await
            .get("hello")
            .await
            .expect("Unable to get the result");
        println!("Got value from the server, {:?}", result);
    }

    Ok(())
}

async fn get_session() -> Client {
    client::connect("127.0.0.1:6379")
        .await
        .expect("Cannot Connect to Server")
}
