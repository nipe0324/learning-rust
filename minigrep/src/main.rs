// ライブラリクレートをバイナリクレートにもってくる
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // NOTE: args()は不正なユニコードを含むとパニックを起こす
    // 取り扱いは大変だが、args_os()を使えば不正なユニコードを受け付けることができる
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parseing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for '{}' in '{}'\n",
        config.query, config.filename
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
