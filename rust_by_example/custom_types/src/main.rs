#![allow(dead_code)]

// struct: 構造体を定義
// enum: 列挙型を定義
// const, static: 定数を作成
fn main() {
    structures();
    enums();
    type_aliases();
    linked_list();
    constants();
}

// 構造体は、データのまとまりに名前をつけれる
// タプル構造体は、タプルに名前をつけれる
// ユニット構造体は、フィールドがない構造体でジェネリックスで役立つ
fn structures() {
    // フィールドをもつ構造体
    struct Point {
        x: f32,
        y: f32,
    }

    // タプル構造体
    struct Pair(i32, f32);

    // ユニット構造体
    struct Unit;

    // 構造体のインスタンス作成時にフィールドを省略できる
    let x = 10.3;
    let y = 0.4;
    let point = Point { x, y };
    println!("point coordinates: ({}, {})", point.x, point.y);
    // point coordinates: (10.3, 0.4)

    // 構造体の更新シンタックスを使うことで、一部のフィールドのみ更新した別のインスタンスをつくれる
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    // second point: (5.2, 0.4)

    // 構造体のフィールドを分解して変数にバインディングできる
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    // ユニット構造体のインスタンス化
    let _unit = Unit;

    // タプル構造体のインスタンス化
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // pair contains 1 and 0.1

    // タプル構造体の分解
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    // pair contains 1 and 0.1

    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    impl Rectangle {
        fn rect_area(&self) -> f32 {
            let Rectangle {
                top_left: point1,
                bottom_right: point2,
            } = self;
            (point2.x - point1.x) * (point1.y - point2.y)
        }
    }

    let rect = Rectangle {
        top_left: Point { x: 1.0, y: 3.0 },
        bottom_right: Point { x: 5.0, y: 1.0 },
    };

    println!("rect ares is {}", rect.rect_area()); // rect ares is 8
}

// 複数の異なる値のいずれかをとりえる型を定義できる
fn enums() {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'", c),
            WebEvent::Paste(s) => println!("pasted \"{}\"", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}", x, y);
            }
        }
    }

    let events = [
        WebEvent::KeyPress('x'),
        WebEvent::Paste("my text".to_owned()),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageLoad,
        WebEvent::PageUnload,
    ];
    for e in events {
        inspect(e);
    }
    // pressed 'x'
    // pasted "my text"
    // clicked at x=20, y=80
    // page loaded
    // page unloaded

    // 暗黙的に識別子は0から始まる
    enum Number {
        Zero,
        One,
        Two,
    }

    // 明示的に識別子に値を設定する
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // 補足：識別子はintegerにキャストできる
    println!("zero is {}", Number::Zero as i32);
    // zero is 0
    println!("one is {}", Number::One as i32);
    // one is 1

    println!("roses are #{:06x}", Color::Red as i32);
    // roses are #ff0000
    println!("violets are #{:06x}", Color::Blue as i32);
    // violets are #0000ff
}

fn type_aliases() {
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                // implブロック内では、Self エイリアスを使える
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    // 型エイリアスを定義
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let x = Operations::Add;
    println!("{}", x.run(1, 2)); // 3
}

fn linked_list() {
    enum List {
        Cons(u32, Box<List>), // 要素と次のノードへのポインタをもつタプル
        Nil,                  // リストの終端
    }

    impl List {
        fn new() -> List {
            List::Nil
        }

        fn prepend(self, elem: u32) -> List {
            List::Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                List::Nil => {
                    format!("Nil")
                }
            }
        }
    }

    let mut list = List::new();
    list = list.prepend(1).prepend(2).prepend(3);
    println!("linked list has length: {}", list.len());
    // linked list has length: 3
    println!("{}", list.stringify());
    // 3, 2, 1, Nil
}

// Rustには2つ種類の定数がある
// const: 変更不可能な値 (一般的なケース)
// static: ミュータブルな変数で 'static ライフタイムで利用される。
//         ミュータブルなstaticな変数にアクセスしたり変更するのは安全ではない。

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn constants() {
    let n = 16;

    println!("This is {}", LANGUAGE); // This is Rust
    println!("The threshold is {}", THRESHOLD); // The threshold is 10
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" }); // 16 is big
}
