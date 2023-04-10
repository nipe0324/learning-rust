fn main() {
    mutability();
    scope();
    shadowing();
    declare_first();
}

// 変数はデフォルトでイミュータブル
// mutを使うことでミュータブルにできる
fn mutability() {
    let _immutable_biding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    // Before mutation: 1

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
    // After mutation: 2

    // イミュータブルな変数は変更しようとするとコンパイルエラーになる
    // _immutable_biding += 1;
}

// 変数はスコープをもち、ブロック内で有効という制約がある。
fn scope() {
    // この変数は、この関数内で有効
    let long_lived_binding = 1;

    {
        // この変数は、このブロック内だけで有効
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }

    // short_lived_bindingはスコープ外なのでアクセスできない
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}

// 既存の変数と同じ名前の新しい変数を宣言し、以降のスコープで元の変数を一時的に隠蔽（shadow）できる
fn shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);
        // before being shadowed: 1

        // シャドーイングしている
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
        // shadowed in inner block: abc
    }

    println!("outside inner block: {}", shadowed_binding);
    // outside inner block: 1
}

// 最初に変数バインディングを宣言し、後で初期化することができる
// ただし、この形式は初期化されていない変数の使用につながる可能性があるためめったに使用されない
fn declare_first() {
    // 変数バインディングのみ宣言
    let a_binding;

    {
        let x = 2;

        // 初期化
        a_binding = x + x;
    }

    println!("a bindning: {}", a_binding);
    // a bindning: 4

    let another_binding;

    // 初期化されていないためコンパイルエラーになる
    // println!("another biding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
    // another binding: 1
}
