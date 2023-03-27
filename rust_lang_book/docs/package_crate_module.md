# パッケージ、クレート、モジュール

- プロジェクトが大きくになるにつれ、複数のモジュール、複数のファイルに分割することでプログラムを整理できる。
- 分割することを通して、機能をグループにまとめたり、実装の詳細を利用者側から隠蔽化することで、コードをより高いレベルで再利用できるようになる。
- Rustのモジュールシステム
  - パッケージ：ある機能群を提供する１つ以上のクレート。`Cargo.toml`をもっている。
    - `src/main.rs`、`src/lib.rs`、`src/bin`あたりは少し特別
  - クレート：ライブラリが実行可能ファイルのどちらか。木構造をしたモジュール群。ex 乱数を扱う`rand`クレート
  - モジュール：パスの構成、スコープ、公開するかどうかを決めれる
  - パス：要素（例えば、構造体や関数やモジュール）に名前をつける方法

## モジュール

- モジュールは`mod`キーワードで定義する。

```rs
// src/lib.rs
// レストランの接客部門（front or house）
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

- 上記のモジュールツリー。ディレクトリのようにツリー構造になっている
- 暗黙的にルートには`crate`になる

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## パス

- モジュールツリー内の要素をみつけるためにパスを使う
- パスには、絶対パスと相対パスの2つがある
  - 絶対パス：クレートの名前か`crate`という文字列から絶対パスで指定する
  - 相対パス：`self`、`super`または今のモジュール内の識別子から相対パスで指定する
- 可視性
  - Rustではあらゆる要素はデフォルトで非公開になる
  - 公開したい場合は、`pub`キーワードで公開できる
  - モジュールの親子関係においては、
    - 親モジュールの要素は子モジュールの非公開要素を使えない
    - 子モジュールの要素は親や祖先モジュールの非公開要素を使える

```rs
// src/lib.rs
mod front_of_house {
    // hostingモジュールにアクセスするには
    // pubキーワードでモジュールを公開する必要がある
    pub mod hosting {
        // add_to_waitlist関数にアクセスするには
        // pubキーワードで関数を公開する必要がある
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パスで指定
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パスで指定
    front_of_house::hosting::add_to_waitlist();
}
```

- 親モジュールから始まる相対パスにしたい場合は、`super`を最初につける

```rs
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 親モジュールから始まるのでsuperをつける
        super::serve_order();
    }

    fn cook_order() {}
}
```

- 構造体ではそれぞれのフィールドごとに`pub`を使って公開するか非公開にするか指定できる
- 列挙型は公開するとすべての列挙子も公開される

```rs
mod back_of_house {
    // 構造体は公開しても、フィールドはデフォルトで非公開
    // そのため、フィールドごとに公開するか指定する必要がある
    pub struct Breakfast {
        pub toast: String, // 公開
        seasonal_fruit: String, // 非公開
    }

  // 列挙型は公開すると、列挙子もすべて公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## `use`キーワード

- `use`でパスをスコープに持ち込むことができ、モジュールの要素にアクセスが楽になる

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// useキーワードでhostingモジュールまで読み込んでいる
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- 慣例では、関数は直接持ち込まず、毎回親モジュールを指定して呼び出すようにする
  - フルパスを繰り返して書くことを抑えつつ、関数がローカルで定義されていないことを明らかにできるため
- また、構造体や列挙型においては、フルパスを書くのが慣例的。これにはとくに理由はないらしい。

```rs
// 構造体や列挙型はフルパスで書く
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- `use`で`as`を使うことで新しい名前をつける。名前が衝突したときの解消につかえる


```rs
use std::fmt::Result;
use std::io::Result as IoResult; // asでIoResultという名前をつけている
// 補足: asではなく、以下のように呼び出してもよいらしい
// use std::fmt;
// use std::io;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
```

## 再公開

- `pub use`を使うことで再公開できる
- 要素を自分たちのスコープに持ち込むだけでなく、他の人がその要素をその人のスコープに持ち込むことも可能になる

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// pub useで再公開されているので、
// 外部のコードも hosting::add_to_waitlist を呼び出すことができる
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

## 外部パッケージ

- 外部パッケージを使うには、`Cargo.toml`の依存関係に追加する

```toml
# Cargo.toml

[dependencies]
rand = "0.8.3"
```

- そして、`use`の行を追加して、スコープに持ち込みたい要素を並べる。


```rs
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret_number: {secret_number}");
}
```

- `use`で複数のパスを読み込みたい場合は、ネストしたパスを使うことで同じ一連の要素を1行でスコープに持ち込める

```rs
// 以下を省略して書ける
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// 以下を省略して書ける
// use std::io;
// use std::io::Write;
use std::io::{self, Write};
```

## `glob`演算子

- `glob`演算子は、パスにおいて定義されているすべての公開要素をスコープに持ち込みたいときに`*`を使う
- `glob`演算子は、どの名前がスコープ内にあり、プログラムで使われている名前がどこで定義されているかわかりづらくなるので、使う際には注意してください。
- よく使われるケースとして、テストのさいに`tests`モジュールを持ち込むときに使われる。

```rs
// std::collections 内のすべての公開要素を現在のスコープに持ち込んでいる
use std::collections::*;
```

## モジュールを複数のファイルに分割する

- 以下のようにファイルのパスがモジュールになる

```rs
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

- また、以下のようにわけることもできる

```rs
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// src/front_of_house.rs
// front_of_houseモジュール内にhostingモジュールが定義される
pub mod hosting;

// src/front_of_house/hosting.rs
// hostingモジュールの関数として定義される
pub fn add_to_waitlist() {}
```
