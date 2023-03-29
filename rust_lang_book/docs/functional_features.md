# 関数型言語の機能：イテレータとクロージャ

- Rustの設計は、多くの既存の言語やテクニックにインスピレーションを得ている。
- 関数型言語の機能に似たRustの機能の一部を説明する

## クロージャ

- Rustのクロージャは、変数に保存したり、引数として他の関数に渡すことのできる匿名関数です。
- クロージャでは、`fn`関数のように引数の型や戻り値の型を注釈する必要はない。

```rs
let expensive_closure = |num| {
    println!("calculated slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
};
```

- クロージャの結果の値をキャッシュするために構造体をつくる方法もあり、メモ化や遅延評価という名前でしられている。

```rs
fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} psitups!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

// clousureの結果をキャッシュする構造体
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 何回呼んでもcalculationの関数は1回しか呼ばれない
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

- クロージャには、環境をキャプチャし、自分が定義されたスコープの変数にアクセスできる能力がある。
- クロージャは、３つの方法で環境から値をキャプチャできる
  - `FnOnce`：所有権を奪う。同じ変数の所有権を2回以上奪えないので`Once`とついている
  - `FnMut`：可変で値を借用するので、環境を変更することができる
  - `Fn`：環境から値を不変で借用する

```rs
fn main() {
    let x = vec![1, 2, 3];

    // クロージャのequal_to_xはx変数を利用できる
    let equal_to_x = |z| z == x;

    // move を利用すると、所有権を移動させることがｄけいる
    // let equal_to_x = move |z| z == x;

    let y =  vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

## イテレータ

- イテレータパターンにより、一連の要素を順番に処理をすることができる。
- イテレータは、各要素を繰り返し、シーケンスが終わったことを決定するロジックの責任を負う。

```rs
fn main() {
    let v1 = vec![1, 2, 3];

    // iterメソッドでイテレータを生成
    for val in v1.iter() {
        println!("Got: {}", val);
    }
}
// Got: 1
// Got: 2
// Got: 3
```

- すべてのイテレータは、標準ライブラリで定義されている`Iterator`トレイトを実装している。

```rs
// 標準ライブラリの`Iterator`トレイトの定義抜粋
pub trait Iterator {
    type Item; // トレイトとの関連型（associated type）

    fn next(&mut self) -> Option<Self::Item>;

    // ...
}
```

- イテレータを消費するメソッドは、消費アダプタと呼ばれる。呼び出しがイテレータの使いこみになる

```rs
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

- 他のイテレータを生成するメソッドは、イテレータアダプタと知られている。
- イテレータアダプタを複数回呼び出しを連結して、複雑な動作を読みやすい形で行うことができる

```rs
#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
```

- イテレータで環境をキャプチャするクロージャを使用する

```rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

- `Iterator`トレイトで独自のイテレータを作成する

```rs
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    // Iteratorトレイトを実装したのでnext()を使用することができる
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    // Iteratorトレイトに実装されているメソッドを使用できる
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
```
