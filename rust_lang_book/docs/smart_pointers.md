# スマートポインタ

- スマートポインタは、ポインタのように振る舞うだけでなく、追加のメタデータと機能があるデータ構造。
- 例えば、参照カウンタ方式のスマートポインタ型が存在し、複数の所有権をもたせることができる
- ※スマートポインタは、Rust特有の機能ではなく、C++や他の言語にも存在する
- スマートポインタは通常、構造体で定義され、`Defer`と`Drop`トレイトを実装している。
- よく使われるスマートポインタとして以下がある
  - `Box<T>`
    - ヒープに値を確保する
  - `<Rc<T>`
    - 複数の所有権を可能にする参照カウント型
    - 複数の所有権を持ちたい場合に利用する
  - `RefCall<T>`
    - 実行時に借用規則を強制する型
- `Box<T>`, `Rc<T>`, `RefCell<T>`の選び方
  - `Box<T>`は、ミュータブルな借用もイミュータブルな借用もコンパイル時に精査できる。
    - `Rc<T>`ではイミュータブルな借用のみコンパイル時に精査できる。`RefCell<T>`ではどちらの借用も実行時に精査される
  - `Rc<T>`は、同じデータにたいして複数の所有権をもてるようになる。
    - `Box<T>`と`RefCell<T>`は単独の所有権
  - `RefCell<T>`は、実行時に精査されるミュータブルな借用を許可するため、値を変更可能にできる。


## ヒープに値を確保する`Box<T>`

- `Box<T>`を使うと、スタックではなくヒープにデータを格納することができる。
- 以下のようなケースでよく利用される
  - コンパイル時にサイズを知ることできない型があり、正確なサイズを要求する文脈でその方の値を使用するとき
  - 多くのデータがあり、その所有権を移したいが、その際にデータがコピーされないようにしたいとき
  - 値を所有する必要があり、特定のトレイトを実装する方であることのみ気にかけているとき


```rs
fn main() {
    // 値5はヒープに確保される
    let b = Box::new(5);
    println!("b = {}", b);
}
// b = 5
```

- `Box<T>`で再帰的な型を利用できるようにする
- 再帰的な型の場合、コンパイラ時にサイズがわからないのでコンパイルエラーになる。
- `Box<T>`を使うことで、再帰的な型を定義できるようになる。


```rs
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // Listは再帰的になっているが、Boxを使うことでusizeになるので計算可能になる
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
    // list: Cons(1, Cons(2, Cons(3, Nil)))
}
```

## 参照カウンタ型の`Rc<T>`

- ときどきグラフデータなど単独の値が複数の所有権を持ちたいときがある。
- Rustで複数の所有権を可能にするために、`Rc<T>`型がある。（Reference Counting：参照カウンタの略）
- `Rc<T>`型は、参照の数を追跡し、値への参照が0になったら、値は解放される。
- ※マルチスレッドで利用する場合は、`Arc<T>`を使う

```rs
use List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("reference count of a: {}", Rc::strong_count(&a));

    // Rc::cloneを呼ぶことでaの所有権をもてる。参照カウンタが1つ増える
    let b = Cons(3, Rc::clone(&a));
    println!("reference count of a: {}", Rc::strong_count(&a));

    // Rc::cloneを呼ぶことでaの所有権をもてる。参照カウンタが1つ増える
    let c = Cons(4, Rc::clone(&a));
    println!("reference count of a: {}", Rc::strong_count(&a));

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
// reference count of a: 1
// reference count of a: 2
// reference count of a: 3
// a: Cons(5, Cons(10, Nil))
// b: Cons(3, Cons(5, Cons(10, Nil)))
// c: Cons(4, Cons(5, Cons(10, Nil)))
```

## 内部可変性を可能にする`RefCell<T>`

- 内部可変性は、ある値へのイミュータブルな参照があるときでさえも、データを変更できるRustでのデザインパターン（普通は、借用規則により許可されていない）

```rs
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // RefCell型で定義する
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut()でミュータブルな参照を取得
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // borrow()でイミュータブルな参照を取得
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```
