fn main() {
    casting();
    literals();
    inference();
    aliasing();
}

// Rustは、プリミティブ型間の暗黙的な型変換をしない。
// ただし、明示的な型変換 (キャスト) は as キーワードを使用して実行できる
fn casting() {
    let decimal = 65.4321_f32;

    // 暗黙的な型変換はコンパイルエラーになる
    // let integer: u8 = decimal;

    // 明示的な型変換
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    // Casting: 65.4321 -> 65 -> A

    println!("1000 as a u16 is: {}", 1000 as u16);
    // 1000 as a u16 is: 1000

    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // 300.0 as u8 is 255
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);
    // nan as u8 is 0
}

// 数値リテラルはサフィックスで型注釈ができる
// サフィックスがなく制約が存在しない場合、
// コンパイラは整数には i32 を使用し、浮動小数点数には f64 を使用する
fn literals() {
    // サフィックスでリテラルの型を指定する
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // サフィックスがない場合は、型が i32, f64 になる
    let i = 1;
    let f = 1.0;

    use std::mem;
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    // size of `x` in bytes: 1
    // size of `y` in bytes: 4
    // size of `z` in bytes: 4
    // size of `i` in bytes: 4
    // size of `f` in bytes: 8
}

// 型推論エンジンは非常に賢く、初期化時だけでなくどのように使われているかも含めて推論する
fn inference() {
    let elem = 5u8;

    let mut vec = Vec::new();

    // コンパイラは vec の型を Vec<u8> と型推論する
    vec.push(elem);

    println!("{:?}", vec); // [5]
}

// type 式により、既存の方に新しい名前をつけられる
// 型名の命名規則はアッパーキャメルケースである必要がある
fn aliasing() {
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
    // 5 nanoseconds + 2 inches = 7 unit?
}
