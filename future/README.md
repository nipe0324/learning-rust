# Future

- Rustで非同期プログラミングをするには、Rust標準の機能だけでなく、`tokio`や`async-std`などのコミュニティクレートを使う必要がある
- 非同期プログラミングを実現するためのそれぞれの役割
  - Rust: `Future`トレイト、`async`、`await`キーワードなどはRust標準ライブラリやコンパイラによって提供されている
  - ユーティリティ: 多くのユーティリティの型、マクロ、関数が`futures`クレートによって提供されており任意で使用できる
  - ランタイム: 非同期コードの実行、IO、タスク生成は、`tokio`や`async-std`などのコミュニティクレートの非同期ランタイムに依存している

## 非同期処理の書き方

```rust
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

#[tokio::main]
async fn main() {
    // let f = greet1();
    // let f = greet2();
    let f = GreetFuture {};

    println!("do something");

    f.await; // ここで関数は実行される。明示的に実行する必要あり
}

// パターン1. asyncキーワード
async fn greet1() {
    println!("Hello, world!");
}

// パターン2. asyncブロック
fn greet2() -> impl Future<Output = ()> {
    async {
        println!("Hello, world!");
    }
}

// パターン3. Futureトレイトのpollメソッドを実装する
struct GreetFuture {}

impl Future for GreetFuture {
    type Output = (); // 非同期処理の返り値を表現する型

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Hello, world!");
        Poll::Ready(())
    }
}
```

## 非同期処理を並列で実行する`join`

```rs
use std::time::Duration;

async fn request_to_service1() -> u64 {
    tokio::time::sleep(Duration::from_secs(2)).await;
    3
}

async fn request_to_service2() -> u64 {
    tokio::time::sleep(Duration::from_secs(5)).await;
    5
}

#[tokio::main]
async fn main() {
    // service1, service2が順に実行されるので 3 + 5 秒かかる
    // let result1 = request_to_service1().await;
    // let result2 = request_to_service2().await;
    // println!("result1: {}, result2: {}", result1, result2)

    // service1, service2が並列に実行されるので 5 秒かかる
    let (result1, result2) = tokio::join!(
        request_to_service1(),
        request_to_service2(),
    );

    println!("result1: {}, result2: {}", result1, result2);
}
```

## `Future`から1つだけ値を取得する`select`

```rs
use std::time::Duration;

async fn f(i: u64) -> u64 {
    tokio::time::sleep(Duration::from_secs(i)).await;
    println!("finish {}", i);
    i
}

#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(f(5));
    let h2 = tokio::spawn(f(4));
    let h3 = tokio::spawn(f(3));
    let Ok((res, remaining)) = futures::future::select_ok(vec![h1, h2, h3]).await else { todo!() };
    println!("res: {:?}, remaining: {:?}", res, remaining);
}
```

## 定期的な実行をする`interval`

```rs
use std::time::Duration;

// 処理の間は必ず1秒あけられる
async fn do_at_one_second_intervals_by_loop() {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        println!("do something!")
    }
}

// 処理に1秒以上かかった場合はすぐに処理が実行される
async fn do_at_one_second_intervals_by_interval() {
    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;


        println!("do something!")
    }
}

#[tokio::main]
async fn main() {
    // do_at_one_second_intervals_by_loop().await;
    do_at_one_second_intervals_by_interval().await;
}
```

## 参考

- [Asynchrounous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Rust の Futureについて](https://blog.tiqwab.com/2022/03/26/rust-future.html)
- [tokio tutorial](https://tokio.rs/tokio/tutorial)
