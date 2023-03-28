use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // NOTE: args()は不正なユニコードを含むとパニックを起こす
    // 取り扱いは大変だが、args_os()を使えば不正なユニコードを受け付けることができる
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parseing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for '{}' in '{}'\n",
        config.query, config.filename
    );

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
