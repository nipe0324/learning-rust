# ジェネリック型、トレイト

## ジェネリック型

- ジェネリックスは、具体的な型や他のプロパティの抽象的な代用品です。
- ジェネリックでは山カッコ`<>`と型引数をつける。よく`type`の省略として`T`が使われる
  - 例: `fn hoge<T>(list: &[T]) -> T`

```rs
// ジェネリック型を使った関数
{
    fn largest<T>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
}

// ジェネリック型を使った構造体とメソッド定義
{
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

- Rustでは、ジェネリック型がるコードをを具体的な型があるコードよりも実行が遅くならないようにしている
- これを実現するために、コンパイラは単相化（monomorphization：コンパイル時に具体的な型をいれる)を行っている

```rs
// 以下のコードに書いた場合
let integer = Some(5);
let float = Some(5.0);

// コンパイルすると、コンパイラが生成した具体的な型の定義に置き換えている
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

let integer = Option_i32::Some(5);
let float = Option_f64::Some(5.0);
```

## トレイト

- トレイトは、抽象的な共通の振る舞いを定義できます
  - ※トレイトは、違いはあるが他の言語でいうインターフェイスと呼ばれる機能に似ている
- `trait`キーワードを使うことでトレイトを定義できます

```rs
// トレイトの定義
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- トレイトを型に実装するには、トレイト名と`for`キーワードを指定して実装する
- 外部のトレイトを外部の型に対して実装できないような制約（孤児のルール: orphan rule）にして、他の人のコードが自分のコードを壊したり、その逆が起きないことを保証してくれる。

```rs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

- トレイトのデフォルト実装を定義できる
- そうすることで、実装者はデフォルト実装を使うか、オーバーライドするかを選べる

```rs
pub trait Summary {
    fn summarize(&self) -> String {
        // デフォルト実装を定義
        String::from("(Read more...)")
    }
}
```

- 引数としてトレイトを使うために、`impl <トレイト>`構文を使える
- `impl <トレイト>`引数にトレイトを実装している何らかの型であると定義できる

```rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

- `impl <トレイト>`の構文は、トレイト境界（trait bound）のシンタックスシュガー

```rs
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

- 複数のトレイト境界を`+`で指定できる

```rs
// impl トレイトでの書き方
pub fn notify(item: &(impl Summary + Display)) {}

// トレイト境界での書き方
pub fn notify<T: Summary + Display>(item: &T) {}
```

- `where`句を使ってより明確なトレイト境界を指定できる

```rs
// トレイト境界での書き方
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// where句を使った書き方
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

- `impl <トレイト>`構文を使い、トレイトを実装している型を返すこともできる
- この書き方は、一種類の型を返す場合のみにしか使えない。複数の型を返したい場合は別の方法で実現する必要がある

```rs
// Summaryを実装している型を返す
fn returns_summarizable() -> impl Summary {
    Tweet {
        ...
    }
}
```

- トレイト境界を利用して、特定のトレイトにだけメソッドを実装できる

```rs
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 型TがDisplayトレイトとPartialOrdトレイトを実装しているときのみ、
// cmp_displayメソッドを実装する
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p1 = Pair::new(1, 2);
    p1.cmp_display();

    let p2 = Pair::new(vec![1], vec![2]);
    // p2.cmp_display(); は呼び出せない
}
```
