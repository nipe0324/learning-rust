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

## 構造体

## Enumとパターンマッチング

## パッケージ、クレート、モジュール

## 一般的なコレクション

## エラー処理

## ジェネリック型、トレイト、ライフタイム

## 自動テストを書く

