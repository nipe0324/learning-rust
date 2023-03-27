# エラーハンドリング

- Rustには例外がない
- Rustでは、エラーは大きく2つに分類できる
  - 回復可能なエラー
    - 問題をユーザに報告し、処理を再試行する。
    - `Result<T, E>`でハンドリング
    - ex. ファイルが見つからないなど
  - 回復不能なエラー
    - バグの兆候
    - `panic!`マクロで
    - ex. 配列の境界を超えてアクセスしたなど

## `panic!`で回復不能なエラー

- `panic!`マクロが呼ばれると、プログラムは失敗のメッセージを表示し、スタックを巻きこどして掃除し、終了する

```rs
fn main() {
    panic!("crash and burn");
}
```

## `Result`で回復可能なエラー

- `Result`型で処理が成功したか失敗したかを判断しエラーハンドリングする。

```rs
// 補足：Result型のEnum定義
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// src/main.rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}
```

- エラー種別ごとのハンドリング

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => {
                    println!("create 'hello.txt' file");
                    fc
                },
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

## エラー時にパニックにする`unwrap`と`expect`

- `Result<T, E>`型には、便利なヘルパーメソッドが定義されている
- `unwrap()`は、`Result`が`Ok`の場合は`Ok`の中身の値を返し、`Result`が`Err`の場合は`panic!`マクロを呼び出す

```rs
// ファイルがない場合はパニックが呼ばれ、ファイルがあればファイルが返される
let f = File::open("hello.txt").unwrap();
```

- `expect()`は、`unwrap()`に似ているが`panic`のエラーメッセージを指定できる

```rs
// ファイルがない場合は"Failed to open hello.txt"というメッセージでパニックが表示され、
// ファイルがあればファイルが返される
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

## エラーを委譲する

- 関数内でエラーが処理する代わりに、関数の呼び出し元にエラー処理を委ねることもある
- その場合は、関数の戻り値で`Result<T, E>`を返す

```rs
use std::io;
use std::io::Read;
use std::fs::File;

// 関数の戻り値で`Result<T, E>`を返している
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let s = read_username_from_file();
    match s {
        Ok(s) => println!("s is {}", s),
        Err(e) => println!("e is {:?}", e)
    }
}
```

- エラーの委譲をショートカットする`?`演算子が便利。
- `?`演算子は、`Result`の値が`Ok`の場合は`Ok`の中身が返ってくる。`Result`の値が`Err`の場合は早期リターンをして`Err`を返す。
- `?`演算子を使うことで、エラーハンドリングのコードが読みやすくなる。
- `?`演算子の制約として、戻り値に`Result<T, E>`を持つ関数でしか利用できない

```rs
// 上記の`read_username_from_file`関数を`?`演算子を使うと以下のように書ける
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
