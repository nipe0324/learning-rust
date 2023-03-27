# 一般的なコレクション

- Rustの標準ライブラリは、コレクションと呼ばれる多くの非常に有益なデータ構造を含んでいる。
- 非常に頻繁に利用されるコレクション
  - ベクタ：可変長の値を並べて保持できる
  - 文字列：文字のコレクション。
  - ハッシュマップ：値を特定のキーと紐付けてくれる。より一般的なデータ構造であるマップの特定の実装
- 標準ライブラリで提供されている他のコレクションは https://doc.rust-lang.org/std/collections/index.html にのっている

## ベクタ

- ベクタは複数の値を保持できるデータ構造。
- `Vec<T>`で定義され、ベクタには同じ型の値しか保持できない。
- `String`の内部的には`Vec<u8>`でUTF-8の文字列を保持している。
- UTF-8の文字列の人が見ている文字数とバイト数は異なるのでややこいよね


```rs
// 新しいベクタを生成
{
    // newで殻のベクタを生成
    let mut v: Vec<i32> = Vec::new();

    // vec!マクロで生成
    let v: Vec<i32> = vec![1, 2, 3];
}

// ベクタを更新する
{
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("v is {:?}", v);
    // v is [5, 6, 7]
}

// ベクタ内の値を順に処理する（イミュータブルな参照）
{
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // 100
    // 32
    // 57
}

// ベクタ内の値を順に処理する（ミュータブルな参照）
{
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v is {:?}", v);
    // v is [150, 82, 107]
}

// ベクタの値を取得する
// []とgetの2つがある
{
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // 存在しない場合、panicをおこす
    let does_not_exist = v.get(100); // 存在しない場合、Option型のNoneを返す
}

// Enumを使えば、ベクタに複数の型をほじできるようになる
{
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row is {:?}", row);
    // row is [Int(3), Text("blue"), Float(10.12)]
}

// ベクタを解放すると、ベクタ内の要素も解放される
{
    let v = vec![1, 2, 3];
} // <- vはここでスコープを抜けるので、解放される
```

## 文字列

- 文字列スライスは`str`や`&str`。これはUTF-8エンコードされた文字列データへの参照。例えば、文字列リテラルは文字列スライスになる
- `String`型は標準ライブラリで提供されており、伸長可能、可変、所有権のあるUTF-8エンコードされた文字列。

```rs
// 新規の文字列を生成する
{
    let s = String::from("initial contents");
}

// to_string()を使って、文字列リテラルから文字列を生成する
{
    let s = "initial contents".to_string();
}

// push_strで文字列を更新する
{
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");
    // s is foobar
}

// +演算子で文字列を連結
{
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1の所有権はムーブされる
    println!("s3 is {s3}");
    // s3 is hello, world!
}

// format!マクロで文字列を連結
{
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {s} by {s1}, {s2}, {s3}");
    // s is tic-tac-toe by tic, tac, toe
}
```

## ハッシュマップ

- ハッシュマップは`HashMap<K, V>`型で、キーと値の対応関係を保持する。

```rs
use std::collections::HashMap;

// 新しいハッシュマップを生成する
{
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores is {:?}", scores);
    // scores is {"Blue": 10, "Yellow": 50}
}

// 別の方法で新しいハッシュマップを生成する
{
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores is {:?}", scores);
    // scores is {"Blue": 10, "Yellow": 50}
}

// 所有権の移動
{
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 所有権がムーブする
    println!("map is {:?}", map);
    // map is {"Favorite color": "Blue"}
}

// ハッシュマップの値にアクセスする
{
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(n) => println!("Blue team score is {n}"),
        None => println!("score is not found"),
    }
    // Blue team score is 10
}

// ハッシュマップを走査する
{
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // Yellow: 50
    // Blue: 10
}

// ハッシュマップの値を上書きする
{
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 上書きされる

    println!("scores is {:?}", scores);
    // scores is {"Blue": 25}
}

// ハッシュマップでキーに値がなかった場合のみ挿入する
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("scores is {:?}", scores);
    // scores is {"Blue": 10, "Yellow": 50}
}

// すでにある値に基づいて値を更新する
{
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insertはミュータブルな参照(&mut V)を返す
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map is {:?}", map);
    // map is {"world": 2, "wonderful": 1, "hello": 1}
}
```
