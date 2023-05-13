# Blog OS

- [Writing an OS in Rust](https://os.phil-opp.com/freestanding-rust-binary/)
- Thanks [phil-opp](https://github.com/phil-opp) :)

## コマンド

ビルド

```
cargo build --target thumbv7em-none-eabihf
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
