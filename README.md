# learning-rust

## Rustの特徴

- Rust is safe, fast, productive
- safe
  - 型システムと所有権により、メモリ管理でメモリ安全でないコードやスレッド間でデータ競合のおこりえるコードはコンパイルエラーになる
- fast
  - 実行可能バイナリにコンパイルするのでランタイム環境が必要ない
  - ガベージコレクタの機構がなく、コンパイル時にメモリの使い方を決めるのでメモリ効率がよい
  - ゼロコスト抽象化を追求している（型や関数でコードを抽象化してもコンパイル時に解決して、実行時に追加のコストがかからない。例. ジェネリック型）
- productive
  - パッケージマネージャやビルド補助ツールなどの開発ツールが豊富

## Rustの主要サイト

- [rust-lang - Github](https://github.com/rust-lang/rust)
- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
  - 日本語版: https://doc.rust-jp.rs/book-ja/
- [crates.io](https://crates.io/): Rustのパッケージ公開サイト
- [Lib.rs](https://lib.rs/): カテゴリごとにクレートを探せる

## Rustのインストールとツール群

- Rustのインストール手順: https://www.rust-lang.org/ja/tools/install
- パッケージ管理 & ビルドツール Cargo
- コードフォーマッタ rustfmt: `cargo fmt`
- 静的解析ツール Clippy: `cargo clippy`
- テスト: `cargo test`
- タスクの自動実行: `cargo-make`: `cargo install --force cargo-make`

## 言語仕様抜粋

- 変数: `let`
  - ※デフォルトイミュータブル、ミュータブルにしたい場合は`mut`を追加
- 文字列: `&str`, `String`
- 固定長配列: `[]`
- 可変長配列: `vec![]`
- タプル型: `()`
- 条件分岐: `if`
- 繰り返し: `for`, `while`, `loop`
- パターンマッチ: `match`
- 構造体: `struct`, `impl`
- 列挙型: `enum`
- 非同期処理: `async`, `await`
- エラー表現: `Option`, `Result`
  - ※nullはない、例外機構はない
- トレイト: `trait`
- マクロ: 関数的マクロ ex. `println!()`、deriveマクロ ex. `#[derive(Debug)]`、アトリビュートマクロ ex. `#[test]`
- 所有権システム
  - 「変数がある値を保持していること」を所有権を持っているという
  - 値あh原則1つの変数のみが保持できるルールで、所有権が移動（ムーブ）してなければ値は破棄され、メモリの開放が行われる

## Cargo

- cargo new <project>: プロジェクトを作成
- cargo build: ビルド
- cargo run: 実行
- cargo fmt: フォーマット
- cargo test: テスト実行

## その他

- Rustを書く人を Rustacean とよんでいるらしい
