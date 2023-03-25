# learning-rust

## Rustの特徴

- パフォーマンス:
  - 実行可能バイナリにコンパイルするのでランタイム環境が必要ない
  - GC機構がなく、コンパイル時にメモリを使い方を決めるのでメモリ効率がよい
  - ゼロコスト抽象化を追求（型や関数でコードを抽象化してもコンパイル時に解決して、実行時に追加のコストがかからない）
- 信頼性
  - 所有権モデルによるメモリ管理でメモリ安全でないコード、スレッド間でデータ競合のおこりえるコードはコンパイルエラーになる
- 生産性
  - パッケージマネージャやビルド補助ツールなどの開発ツールが豊富

## Rustのインストールとツール群

- Rustのインストール手順: https://www.rust-lang.org/ja/tools/install
- パッケージ管理 & ビルドツール Cargo
- コードフォーマッタ rustfmt: `cargo fmt`
- 静的解析ツール Clippy: `cargo clippy`
- テスト: `cargo test`
- タスクの自動実行: `cargo-make`: `cargo install --force cargo-make`
- [crates.io](https://crates.io/): Rustのパッケージ公開サイト
- [Lib.rs](https://lib.rs/):  カテゴリごとにクレートを探せる

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
