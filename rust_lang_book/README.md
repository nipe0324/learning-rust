# rust-lang book

[The Rust Programming Language](https://doc.rust-lang.org/stable/book/)の[日本語版](https://doc.rust-jp.rs/book-ja/)

## Cargo

CargoはRustのビルドシステム兼パッケージマネージャ。  
Cargoはコードのビルド、依存ライブラリのダウンロードやビルドなどを扱ってくれる。

### Cargoの主要なコマンド

- `cargo new` プロジェクトを作成
- `cargo build` プロジェクトをビルド
  - `cargo build --release` リリースに向けた最適化した状態でビルドする
- `cargo run` プロジェクトのビルドと実行
- `cargo check` バイナリを生成せず、プロジェクトをビルド

### `Cargo.toml`

`Cargo.toml`はCargoのTOML形式の設定ファイル。

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- `[package]` 
- `[dependencies]` 依存するクレート（Rustではコードのパッケージのことをいう）を管理する

Cargoの詳細: https://doc.rust-lang.org/cargo/

## Rustの基本文法

### 変数と可変性

- 変数は`let`で宣言する
- 変数は、デフォルトでイミュータブル（不変）。`mut`をつけることでミュータブルにできる（可変）

```rs
fn main() {
    // 変数はデフォルトでイミュータブル（不変）になる。
    let x = 3;
    println!("the value of x is: {x}");

    // 変数はmutをつけることでミュータブル（可変）になる
    let mut y = 4;
    println!("the value of y is: {y}");
    y = 5;
    println!("the value of y is: {y}");
}

// [Output]
// the value of x is: 3
// the value of y is: 4
// the value of y is: 5
```

イミュータブルにするか、ミュータブルにするか？

- イミュータブルにすると、コードを読み書きするときに、どこでどうやって値が変化しているかを追いかける必要がなくなり、可読性があがり、バグも発生しづらくなる。
- しかし、実行速度が重要かつ大きなデータを扱う場合、ミュータブルにして変更できるほうが、インスタンスコピー時に新しくメモリ割り当てをされたインスタンスを返すイミュータブルよりも速くなる。

シャドーイング

- 前に定義した変数と同じ名前の変数を新しく宣言できる。最初の変数は2番目の変数にシャドーイングされたといいます。

```rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
// [Output]
// The value of x in the inner scope is: 12
// The value of x is: 6
```

- シャドーイングのおかげで、異なる名前を考えなくてよくなる
- ※注：同じ名前を使い回すことを推奨しているかはわからない

```rs
let spaces = "    ";
let spaces = spaces.len();
```

### 定数

- 定数は`const`で宣言する
- 定数の一般的な利用ケースとして、マジックナンバーに対して定数として名前をつける。可読性を高めつつ、値の変更時には一箇所変更すればよいだけになる。

```rs
const MAX_POINTS: u32 = 100_100;
```

### データ型

- データ型にはスカラー型と複合型の2つがある
- Rustでは型推論をしてくれる。複数の型が推論される場合は、型注釈をつける必要がある
  - `let guess: u32 = "42".parse().expect("Not a number!");`
- Rustに`null`はない
- スカラー型は、単独の値を表し、整数、浮動小数点数、論理値、文字の4つがある
  - 整数型: `i32, u32, i64, u64, ...`のように符号ありなしとbitサイズで宣言。基準型は`u32`。
  - 浮動小数点型：`f32`と`f64`の2つで宣言。基準型は`f64`。
  - 論理値型：`bool`で宣言。値は`true`と`false`のみ
  - 文字型：`char`で宣言。値は`"`ではなく`'`で指定する。文字型はUnicodeのスカラー値を表す。
- 複合型は、複数の値を１つの型にまとめられる
  - タプル型：丸括弧(`()`)とカンマ（`,`）を利用しての値のリストを生成できる。
  - 配列型：カギ括弧(`[]`)とカンマ（`,`）を利用しての複数の値のコレクションを生成できる。
    - 可変長にしたい場合は、ベクタ型を使う

### 関数

- `fn`により関数を定義する。
- 関数名にはスネークケースを使うのが慣習になっている。
- 補足：`main`関数はプログラムのエントリーポイント（プログラム実行時に最初に走る関数）になる。
- 関数の戻り値は`return`で値を返せる。多くの関数は最後の式を暗黙的に返すので`return`を書かないことが多い。

```rs
fn main() {
    let result = plus(3, 2);
    println!("The value of result is: {result}")
}

fn plus(a: i32, b: i32) -> i32 {
    a + b    
}
// [Output]
// The value of result is: 5
```

式と文

- 文（Statement）は、なんらかの動作をして、値を返さない命令です。
- 式（Expression）は、なんらかの動作をして、値が返される命令です。
- Rustは式指向の言語

```rs
fn main() {
    let y = {
        let x = 3; // statement
        x + 1 // expression
    };

    println!("The value of y is: {y}");
}
// [Output]
// The value of y is: 4
```

### コメント

- コメントは`//`で記載する

```rs
// this is comment
```

### 制御フロー

- 条件分岐：`if`式
- コードを繰り返す
  - `while`や`loop`でループ、`continue`で次のループに移動、`break`でループから抜ける
  - `for`を使ってコレクションの要素にアクセスできる

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
// [Output]
// the value is: 10
// the value is: 20
// the value is: 30
// the value is: 40
// the value is: 50
```

## 所有権を理解する

- 所有権はRustで最もユニークな機能で、所有権によりガベージコレクタなしでメモリ安全性を保証している。
- 所有権のルール
  - Rustの各値は、所有者と呼ばれる変数と対応している
  - 一度に存在できる所有者は1人だけ
  - 所有者がスコープから外れたら、値は破棄される
- ref: https://doc.rust-jp.rs/book-ja/ch04-00-understanding-ownership.html

```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 所有権の移動

    println!("{}, world!", s2);
}
```

メモリの解放

- GC付き言語：プログラマをメモリを気にしなくてよい。ガベージコレクタで頑張っているが
- 明示的にメモリ管理する言語：プログラマが明示的にメモリの確保と解放のコードを呼び出す。allocateとfreeを完璧に1対1にしないといけずバグやメモリリークを起こしやすい
- Rust：所有している変数がスコープを抜けたら、メモリは自動的に返還される。変数がスコープを抜けるときにRustは特別な関数である`drop`関数が自動的に呼ばれる。

所有権の移動：ムーブ

- 値が新たに他の変数から参照されると、所有権が移動する。ムーブされたと表現する
  - ※スタック上につまれている整数や文字列リテラルなどのデータの場合はコピーする。
- Rustは所有権を管理することで、メモリ解放を自動的に実施できるようにしている
- 補足：Rustで自動的に"deep copy"が行われることはない
  - そのため、コピーはスタック領域にポインタを作成するオーバーヘッドがあるだけなので、実行時性能は悪くない
  - ヒープ領域のデータに対しても、"deep copy"をしたい場合は、`clone`メソッドを使う
- 関数に変数を渡すと、代入のようにムーブやコピーがされる。戻り値で値を返すことで所有権は移動する。

```rs
fn main() {
    let s1 = String::from("hello");       // s1がスコープに入る
    takes_ownership(s1);                  // s1の所有権がムーブする。以下の行ではs1は有効ではない

    let s2 = String::from("world");       // s2がスコープに入る
    let s2 = takes_and_gives_back(s2);    // s2はtakes_and_gives_backにムーブされるが、戻ってくる
    println!("The value of s2 is: {s2}"); // s2の所有権は戻ってきているので参照できる

    // i32はムーブではなくコピー
    let x : i32 = 5;                      // xがスコープに入る
    makes_copy(x);                        // i32はコピーなので、この後にxを使っても大丈夫
    println!("The value of x is: {x}");

} // ここで、s2はスコープを抜け、ドロップされる
  // s1もスコープを抜けるが、ムーブされているので何も起きない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。メモリが解放される。

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

// [Output]
// hello
// The value of s2 is: world
// 5
// The value of x is: 5
```

参照と借用

- 関数の引数で`&`を使って参照で渡すことで、所有権が移動しないようにできる。※内部的にはポインタのポインタぽい
- また、参照渡しされた関数側は借用している立場なので、借用した何かを変更しようとするとコンパイルエラーになる

```rs
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &を使うことで所有権がムーブしない

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // 借用している値を変更できない
    // `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // s.push_str(", world");
    s.len()
} // 所有権がムーブしていないので、dropはされない

// [Output]
// The length of 'hello' is 5.
```

- `mut`を使うことで、参照を可変にすることができる。
- 制約として、特定のスコープ内では、あるデータに対して、１つしか可変な参照を持てない。
  - この制約により、コンパイル時にデータ競合を防ぐことができる。

```rs
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("some_string is {some_string}");
}

// [Output]
// some_string is hello, world
```

- 所有権のない別のデータ型としてスライス型がある。
- スライスは、コレクション全体ではなく、その内の一連の要素を参照することができる。（内部的にはポインタと長さ）

```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // コンパイルエラーが発生する: cannot borrow as mutable
    println!("the first word is: {}", word);
}

// [Output]
// the first word is: hello
```

補足：スタックとヒープ

- Rustのようなシステムプログラミング言語においては、値がスタックに積まれるかヒープに置かれるかは、言語の振るまいや特定の決断を下す理由に大きな影響を与える。
- メモリ管理として、スタックでポインタ、ヒープ領域で実行時の可変長の実データを保持する
- スタック
  - last in, first out。スタックにpushする。スタックからpopする
  - データの配置や取得が常にスタックの一番上のため、データを探す手間がないため、スタックは高速。
  - ※スタックを高速に保つためには、スタック上のデータはコンパイル時にはすべて既知の固定サイズでなければならない
- ヒープ
  - コンパイル時にサイズがわからなかったり、サイズが可変のデータは、ヒープに格納される
  - ヒープ領域からメモリを確保するため、スタックに比べて遅い。通常はメモリを確保してポインタを返す

## 構造体

- 構造体は、意味のあるグループを形成する複数の関連した値をまとめ、名前付けできる独自のデータ型です。
- `struct`キーワードで構造体を定義する。
- 構造体を使用することで、関連のあるデータを相互に結合させたまま、各部品に名前を付け、コードを明確にすることができる。
- また、メソッドにより、構造体のインスタンスが行う動作を指定したり、関連関数により構造体に関連する関数の名前空間わけすることができる。

補足：所有権の移動と参照の検証

```rs
#[derive(Debug)]
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("drop MyStruct: {:?}", self);
        // self.drop(); // 書き方あっているかわからん
    }
}

// 所有権が移動する
fn do_sometihng(s: MyStruct) {
    println!("called do_something: {:?}", s);
} // s1のdropがここで呼ばれる

// 参照で渡すので所有権が移動しない
fn borrow_sometihng(s: &MyStruct) {
    println!("called borrow_sometihng: {:?}", s);
}

fn main() {
    println!("1. move ownership");
    let s1 = MyStruct {};
    do_sometihng(s1);
    println!("after do_something\n");

    println!("2. reference ownership");
    let s2 = MyStruct {};
    borrow_sometihng(&s2);
    println!("after borrow_sometihng");
} // s2のdropがここで呼ばれる

// [Output]
// 1. move ownership
// called do_something: MyStruct
// drop MyStruct: MyStruct
// after do_something

// 2. reference ownership
// called borrow_sometihng: MyStruct
// after borrow_sometihng
// drop MyStruct: MyStruct
```

```rs
// ユーザアカウントに関する情報を保持する構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
- 構造体のインスタンス生成は、構造体名とブロック(`{}`)で生成できる。
- フィールド名と変数数名が同じ場合はフィールド省略記法が使える

```rs
// フィールド名と変数数名が同じ場合はフィールド省略記法が使える
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
```

- 構造体の更新記法を利用で他のインスタンスから新しいインスタンスを生成できる

```rs
let user2 = User {
    username: String::from("another123"),
    email: String::from("another@example.com"),
    ..user1 // 更新帯の更新記法
};
```

- フィールドに紐付けられた名前がなく、フィールドの型だけの構造体をタプル構造体という

```rs
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
let white = Color(255, 255, 255);
```

- 構造体の内容のデバッグ出力

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
    // rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1);
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}
```

- 構造体にはメソッドを定義できる
- `impl`ブロックで内に`fn`でメソッドを定義できる
- メソッドの場合、第一引数は`self`で呼び出し元の構造体自体を表す

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- Rustのメソッド呼び出しでは、自動参照および参照外しが自動で行われる
- `object.something()`と呼び出すと、コンパイラはメソッドのシグニチャに合うように自動的に、`&`か`&mut`か`*`を付与する
- 受け手とメソッド名が与えられれば、コンパイラは確実にメソッドが「読み込み専用(`&self`)」か、「書き込みもする(`&mut self`)の」か、 「所有権を奪う(&self)のか」判断できるわけです。

```rs
// 前者はコンパイラが自動で解決してくれるのでどっちも同じになる
p1.distance(&p2);
(&p1).distance(&p2);
```

- `impl`ブロック内に`self`を引数にとらない関数を定義できる。構造体に関連付けられているのでこれを関連関数と呼ばれる
- 関連関数は、構造体の新規インスタンスを返すコンストラクタによく使用される

```rs
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let rect = Rectangle::square(40);
```

## Enumとパターンマッチング

列挙型の定義

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


Option型

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

match制御フロー

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

## パッケージ、クレート、モジュール

## 一般的なコレクション

## エラー処理

## ジェネリック型、トレイト、ライフタイム

## 自動テストを書く

