# 自動テスト

- テストは、テスト対象のコードが想定された方法で機能していることを検証する
- テスト関数の本体は典型的に、以下の3つの動作を行う
  - 1. 必要なデータや状態をセットアップする
  - 2. テスト対象のコードを走らせる
  - 3. 結果が想定どおりであることをアサートする

## テスト関数の作成

- Rustにおけるテストは`test`属性で注釈された関数のことで、`fn`の前に`#[test]`を付け加える
- そして、`cargo test`コマンドでテストを実行する。
- コンパイラは`test`属性で注釈された関数を走らせるテスト用バイナリをビルドし、各テスト関数が成功したか失敗したかを検証する

```rs
// テスト対象のコード
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// テストコード
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

## アサーション

- `assert!`：bool値を検証
- `assert_eq!`：値が同ことをか検証
- `assert_nq!`：値が異なることを検証

## テスト用の属性

- `should_panic`属性はでパニックを確認できる。
- また、`expected`引数を追加することでパニックのメッセージ内容を検証でｋりう

```rs
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## テストの実行方法

- 複数のテストを実行sルウト気、標準ではスレッドを使用して並行に実行される。
- テストは同時に実行されるので、テストが相互に依存しないように注意する必要がある

```sh
# 直列でテストを実行する（テストスレッドが1つ）
$ cargo test -- --test-threads=1
```

- 成功したテストでは標準出力がでないので、出したい場合は`nocapture`を指定する

```sh
$ cargo test -- --nocapture
```

- 特定のテストを実行する


```sh
# テスト名を指定することで、指定したテストのみ実行できる（テスト名の部分一致ぽい）
$ cargo test it_works
```

## テストの体系化

- Rustのコミュニティでは、テストを単体テストと結合テストの2つの大きなカテゴリで捉えている
- 単体テストは、小規模で個別に1回に1モジュールをテストし、非公開のインターフェイスもテストすることもある
- 結合テストは、完全にライブラリ外になり、他の外部コード同様に自分のコードを使用し、公開インターフェイスのみ使用して、1テストにつき複数のモジュールを使用することもある
  
## 単体テスト

- 単体テストの目的は、残りのコードから切り離して各単位のコードをテストし、 コードが想定通り、動いたり動いていなかったりする箇所を迅速に特定することです。
- 単体テストは、テスト対象となるコードと共に、srcディレクトリの各ファイルに置きます。
- 慣習は、各ファイルに`tests`という名前のモジュールを作り、テスト関数を含ませ、そのモジュールを`cfg(test)`で注釈することです。`cfg[test]`の注釈は、コンパイラに`cargo test`を走らせたときだけテストコードをコンパイルするように指示する。

```rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

- テストコミュニティ内で非公開関数を直接てすとするべきかどうかについては議論がありますが、どちらの考えであったとしても、Rustでは非公開関数をテストすることは可能です。テストするかどうかはプログラマに委ねられている。

```rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## 結合テスト

- Rustにおいて、結合テストは完全にライブラリ外のもので、ライブラリの公開APIの一部である関数しか呼び出すことができない。
- 目的は、 ライブラリのいろんな部分が共同で正常に動作しているかをテストすること。
- 結合テストを作成するには、 `tests`ディレクトリが必要になります。
- Cargoが`tests`ディレクトリを特別扱いするので、結合テストのファイル内では、`#[cfg(test)]`で注釈する必要はない。

```rs
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}


// tests/integration_test.rs
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

- `tests/<module_name>/mod.rs`を作成することで、結合テスト内のサブモジュールを使うことができる


```rs
// tests/common/mod.rs
pub fn setup() {
    // do setup
}

// tests/integration_test.rs
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    // サブモジュールの利用
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

- 結合テストから`src/main.rs`に定義された関数をインポートすることはできない。
- そのため、バイナリを提供するRustのプロジェクトは、`src/lib.rs`ファイルにロジックを書き、それを呼び出す単純な`src/main.rs`があるというケースが多い。
