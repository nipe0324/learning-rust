use std::env;

fn main() {
    // NOTE: args()は不正なユニコードを含むとパニックを起こす
    // 取り扱いは大変だが、args_os()を使えば不正なユニコードを受け付けることができる
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
