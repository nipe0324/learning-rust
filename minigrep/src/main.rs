use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

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

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
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

// Box<dyn Error> は、関数がErrorトレイトを実装する型を返すことを意味し、
// 戻り値の型を具体的に指定しなくても良くなる
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(()) // 副作用のためだけに呼び出していると示唆する慣習的な書き方
}
