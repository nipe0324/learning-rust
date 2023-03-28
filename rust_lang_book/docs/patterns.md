# パターンとマッチング

## パターンが使用される部分

Rustにおいてパターンはいろんな箇所に出現します。

- `match`式を使って、パターンに値が合致したら実行される式を指定する。

```rs
match VALUE {
    PERTTERN => EXPRESSION,
    PERTTERN => EXPRESSION,
    PERTTERN => EXPRESSION,
}
```

- 条件分岐の`if let`式は1つのパターンに合致する`match`と同様のものを書く省略法と使用できる。
- また、オプションとして、パターンに合致しないときの`else`も用意できる

```rs
fn main() {
    let favorite_color: Option<&str> = None;

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else {
        println!("Using blue as the background color");
    }
}
```

- `whle letの`条件分岐ループは、パターンが合致し続ける限り、`while`ループを走らせる

```rs
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

- `for`ループでもパターンが利用できる

```rs
fn main() {
    let v = vec!['a', 'b', 'c'];

    // enumerate()はタプルを生成せいており、(index, value)とマッチさせている
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
```

- `let`文でもパターンが利用できる

```rs
fn main() {
    let (x, y, z) = (1, 2, 3);

    println!("x is {x}, y is {y}, z is {z}");
}
// x is 1, y is 2, z is 3
```

- 関数の引数でもパターンが利用できる

```rs
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
    // Current location: (3, 5)
}
```

## パターン記法

- パターンのいろろな記法：https://doc.rust-jp.rs/book-ja/ch18-03-pattern-syntax.html
