# Cargo

CargoはRustのビルドシステム兼パッケージマネージャ。  
Cargoはコードのビルド、依存ライブラリのダウンロードやビルドなどを扱ってくれる。

## Cargoの主要なコマンド

- `cargo new` プロジェクトを作成
- `cargo build` プロジェクトをビルド
  - `cargo build --release` リリースに向けた最適化した状態でビルドする
- `cargo run` プロジェクトのビルドと実行
- `cargo check` バイナリを生成せず、プロジェクトをビルド

## `Cargo.toml`

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
