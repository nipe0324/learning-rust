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

## イテレータ

- 一連の要素を処理する方法
