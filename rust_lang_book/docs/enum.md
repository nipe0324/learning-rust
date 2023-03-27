# Enumとパターンマッチング

## Enumの定義

- 列挙型は`enum`とも言われ、取りうる値を列挙することで、型を定義できる。

```rs
// enumで列挙型を定義
enum IpAddKind {
    V4,
    V6,
}

// ::で列挙型を参照できる
let four = IpAddKind::V4;
let six = IpAddKind::V6;
```

- 列挙型には直接データを格納することもできる
- また、紐付けるデータ型はそれぞれ異なったデータ型を定義できる

```rs
// 列挙型に異なるデータ型を定義
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 列挙子に応じて引数を変える
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

## Enumのメソッド

- 列挙型も構造体のように`impl`キーワードでメソッドを定義できる

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // メソッド本体はここに定義される
    }
}

let m = Message::Write(String::from("hello"));
m.call();
}
```

## Option型

- RustにはNullがない。Nullの代わりに、値が存在するか存在しないかという概念をenumの`Option<T>`で定義している。

```rs
enum Option<T> {
    Some(T), // 値が存在することを表す
    None,    // 値が存在しないことを表す
}
```

- `Option<T>`は有益すぎて、列挙子の`Some`と`None`は`Option::`という接頭辞なしに利用できる

```rs
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

## match制御フロー

- `match`を使うと、値のパターンに対して値を比較し、マッチしたパターンに応じてコードを実行できる

```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- `Option<T>`で値があるときに処理をしたい場合は、`match`式と以下のように組み合わせることができる。

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

- `if let`で制御フローを簡潔にかける

```rs
fn main() {
    let some_u8_value = Some(0u8);

    // matchで書いたパターン
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let で書き換えたパターン
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
```
