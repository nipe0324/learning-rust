# 所有権を理解する

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

## メモリの解放

- GC付き言語：プログラマをメモリを気にしなくてよい。ガベージコレクタで頑張っているが
- 明示的にメモリ管理する言語：プログラマが明示的にメモリの確保と解放のコードを呼び出す。allocateとfreeを完璧に1対1にしないといけずバグやメモリリークを起こしやすい
- Rust：所有している変数がスコープを抜けたら、メモリは自動的に返還される。変数がスコープを抜けるときにRustは特別な関数である`drop`関数が自動的に呼ばれる。

## 所有権の移動：ムーブ

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

## 参照と借用

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

## 補足：スタックとヒープ

- Rustのようなシステムプログラミング言語においては、値がスタックに積まれるかヒープに置かれるかは、言語の振るまいや特定の決断を下す理由に大きな影響を与える。
- メモリ管理として、スタックでポインタ、ヒープ領域で実行時の可変長の実データを保持する
- スタック
  - last in, first out。スタックにpushする。スタックからpopする
  - データの配置や取得が常にスタックの一番上のため、データを探す手間がないため、スタックは高速。
  - ※スタックを高速に保つためには、スタック上のデータはコンパイル時にはすべて既知の固定サイズでなければならない
- ヒープ
  - コンパイル時にサイズがわからなかったり、サイズが可変のデータは、ヒープに格納される
  - ヒープ領域からメモリを確保するため、スタックに比べて遅い。通常はメモリを確保してポインタを返す
