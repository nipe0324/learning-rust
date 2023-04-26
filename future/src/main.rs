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
