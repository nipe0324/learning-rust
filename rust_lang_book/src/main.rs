use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let s = read_username_from_file();
    match s {
        Ok(s) => println!("s is {}", s),
        Err(e) => println!("e is {:?}", e)
    }
}
