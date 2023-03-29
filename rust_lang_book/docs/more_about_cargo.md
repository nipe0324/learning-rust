# CargoとCrates.ioをより詳しく

- リリースプロファイルでビルドをカスタマイズする
- [crates.io](https://crates.io/)でライブラリを公開する
- ワークスペースで巨大なプロジェクトを体系化する
- [crates.io](https://crates.io/)からバイナリをインストールする
- 独自のコマンドを使用してCargoを拡張する

## リリースプロファイルでビルドをカスタマイズ

- Rustにおいて、リリースプロファイルは、プログラマがコードのコンパイルオプションについてより制御可能にしてくれる定義済みのカスタマイズ可能なプロファイルです。
- Cargoには主に2つのプロファイルが存在する
  - 開発用の`dev`プロファイルは`cargo build`コマンドを実行したときに使用される
  - リリース用の`release`プロファイルは`cargo build --release`コマンドを実行したときに使用される

```rs
// 開発用
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs

// リリース用
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

- `Cargo.toml`ファイルに`[profile.*]`セクションが存在しないときにはデフォルト値が適用される。もちろん上書きすることが可能
- デフォルトのプロファイルの設定は https://doc.rust-lang.org/cargo/reference/profiles.html に記載されている。

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Crates.ioにクレートを公開

- 役に立つドキュメンテーションコメント（`///`を行うと、自動的にHTMLドキュメントを生成する。
- そして、`cargo doc --open`で現在のクレートのドキュメント用のHTMLと、自分のクレートが存在しているすべてのドキュメントを構築し、Webブラウザで閲覧できる。
- また、ドキュメンテーションコメントに` # Examples`のコードブロックを追加すると、`cargo test`時にテストとして実行してくれる

```rs
/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

- クレートのルートファイル（慣例的には`src/lib.rs`）やモジュールの内部で、クレートやモジュール全体に`//!`でドキュメントをつける

```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

- `crates.io`のアカウントをセットアップする
- 新しいクレートにメタデータを追加する
- `cargo publish`で`crates.io`に公開する
- 既存のクレートの子バージョンの公開はセマンティックバージョンルールで公開する
- 以前のバージョンのクレートを削除するときは`cargo yank --vers x.x.x`で削除する

## Cargoのワークスペース

- プロジェクトの開発が進むにつれて、ライブラリクレートの肥大化が続き、複数のライブラリクレートにパッケージを分割したくなってくる。
- このケースにおいて、Cargoは**ワークスペース**という関連のある複数のパッケージを管理するのに役立つ機能を提供している

```rust
// Cargo.toml
[workspace]

members = [
  "adder",
  "add-one",
]
```

- ワークスペース内にクレートを追加する

```sh
$ cargo new --bin adder
     Created binary (application) `adder` package

$ cargo new add-one --lib
     Created library `add-one` package
```

- 現在のワークスペースのディレクトリ階層

```sh
add
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── adder
    ├── Cargo.toml
    └── src
        └── main.rs
```

- `add`ライブラリクレートを実装する

```rs
// add/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
```

- `adder`バイナリクレートから`add`を使用する

```sh
# adder/Cargo.toml

# ...

[dependencies]

add-one = { path = "../add-one" }
```

```rs
// adder/src/main.rs

extern crate add_one;

fn main() {
    let num = 10;

    println!("{} plus one is {}!", num, add_one::add_one(num));
}
```

- バイナリクレートを実行する

```rs
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
10 plus one is 11!
```

## Crates.ioからバイナリをインストール

- `cargo install`コマンドにより、バイナリクレートをローカルにインストールして利用できる。
- 補足として、バイナリターゲットを持つパッケージのみインストールできる
- インストール先は通常`$HOME/.cargo/bin`配下におかれる

```sh
$ cargo install ripgrep
Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ripgrep v0.3.2
 --snip--
   Compiling ripgrep v0.3.2
    Finished release [optimized + debuginfo] target(s) in 97.91 secs
  Installing ~/.cargo/bin/rg

$ rg --help
```

## 独自のコマンドでCargoを拡張

- Cargoを変更することなく、新しいサブコマンドで拡張できるように設計されている
- `$PATH`にあるバイナリが`cargo-something`という名前なら、`cargo something`を実行することで、Cargoのサブコマンドであるかのように実行できる。
