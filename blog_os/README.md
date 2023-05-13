# Blog OS

- [Writing an OS in Rust](https://os.phil-opp.com/freestanding-rust-binary/)
- Thanks [phil-opp](https://github.com/phil-opp) :)

## コマンド

セットアップ

```sh
# Nightビルドの利用
# https://doc.rust-jp.rs/book-ja/appendix-07-nightly-rust.html
rustup install nightly
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-apple-darwin 
rustup toolchain list

# ブータブルディスクイメージのツールの設定
cargo install bootimage
rustup component add llvm-tools-preview

# QUMUのインストール for Mac
brew install qemu
```

ビルド・起動

```sh
# ブータブルディスクイメージの作成（bootloaderとカーネルを合体させている）
cargo bootimage --target x86_64-blog_os.json

# QEMUで起動する
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```

## 用語

- ベアメタル
  - 基盤となるOSなしで動く実行環境のこと
  - OSが提供している、スレッド、ヒープメモリ、ネットワーク、乱数、標準出力、特定のハードウェアを必要とする機能は使えない
  - 今回のようにRustでOSカーネルを書くために必要
- アンワインド
  - デフォルトでは、パニックが起きた場合にはRustはアンワインドを使用してすべてのスタックにある変数のデストラクタを実行する
  - これにより、使用されている全てのメモリが確実に解放され、親スレッドはパニックを検知して実行を継続できる。
  - アンワインドは複雑で、いくつかのOS特有のライブラリを必要とするので、Blog OSでは無効化している
- ブードプロセス
  - コンピュータを起動すると、マザーボードのROMに保存されたファームウェアのコードを実行する。
  - このコードは、起動時の自己テストを実行し、使用可能なRAMを検出し、CPUとハードウェアを事前に初期化する。
  - その後、ブータブルディスクを探し、OSのカーネルを起動させる。
  - x86には2つのファームウェアの標準規格があり、BIOS（古く時代遅れだがシンプルで1980年代からサポートされている）とUEFI（セットアップが複雑だが、モダンでより多くの機能をもっている）がある。
