// Rustは struct や enum などもトレイトを使うことで変換できる
// 一般的な変換は From と Into トレイトを使用する。
fn main() {
    from_and_into_trait();
    try_from_and_try_into();
    converting_to_string();
    parsing_a_string();
}

// From トレイトを使用すると、別の型から自身の型に変換する方法を定義できる
// プリミティブ型と共通型を変換するための標準ライブラリ内には、このトレイトの多数の実装がある
fn from_and_into_trait() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("my_str is {:?}", my_str); // my_str is "hello"
    println!("my_string is {:?}", my_string); // my_string is "hello"

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(10);
    println!("My number is {:?}", num); // My number is Number { value: 10 }

    // From トレイトを実装してあれば、 into()を呼べる
    // 一般的に Into トレイトを利用するのは、コンパイラが明示的に型変換できないときに定義する
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num); // My number is Number { value: 5 }
}

// TryFrom と TryInto トレイトは、Result型を返すことで型変換の失敗を返せる
fn try_from_and_try_into() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

// ToString トレイトを利用して、Stringに変換する
// fmt::Display トレイトを実装すると、自動的に ToString トレイトと print! に対応できる
fn converting_to_string() {
    use std::fmt;

    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    // Circle of radius 6
}

// 文字列を数値に変換することはよくあり、parse関数を使うか、turbofish構文を使用して変換できる
// FromStr トレイトが実装されていれば文字列を指定された型に変換する
// 標準ライブラリの多数の型に対して、FromStr トレイとは実装されている
fn parsing_a_string() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);
    // Sum: 15
}
