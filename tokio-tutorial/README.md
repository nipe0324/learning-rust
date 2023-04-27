# Tokio Tutorial

- https://tokio.rs/tokio/tutorial
- Build Mini-Redis for learning Tokio.

## Tokio

- TokioはRustの非同期ランタイムの1つ
- 非同期コードの実行のためのマルチスレッドランタイム
- IO-bound向け。CPU-boundの場合は、https://github.com/rayon-rs/rayon とかがよいかもと言ってる

## Run

```
# run server
cargo run --bin server

# run client
cargo run --bin client
```

## Memo

```rs
#[tokio::main]
async fn main() {
    println!("Hello world");
}

// 以下のコードと同じ
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
        })
}
```
