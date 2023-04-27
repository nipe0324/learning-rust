use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // コネクションを確立する
    let mut client = client::connect("127.0.0.1:6379").await?;

    // "hello"というキーに"world"という値をセットする
    client.set("hello", "world".into()).await?;

    // "hello"というキーの値を取得する
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
