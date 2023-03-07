use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the local mini-redis server.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with a value of "tokio".
    client.set("hello", "tokio".into()).await?;

    // Get the value of the key "hello".
    let res = client.get("hello").await?;

    println!("Got value from the server: result={:?}", res);

    Ok(())
}
