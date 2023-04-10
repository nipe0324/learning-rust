/*

Scalar Types
- Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
- Floating point: f32, f64
- char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
- bool either true or false
- The unit type (), whose only possible value is an empty tuple: ()

Compound Types
- Arrays like [1, 2, 3]
- Tuples like (1, true)

 */

fn main() {
    variables();
    literals_and_operators();
    tuples();
    array_and_slices();
}

fn variables() {
    // 変数は型注釈ができる
    let logical: bool = true;

    let a_float: f64 = 1.0; // 標準的な型注釈
    let an_integer = 5i32; // サフィックスの型注釈

    // もしくは、デフォルトの型が利用される
    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    // 型推論がされる
    let mut inferred_type = 12; // 他の行からi64に推論される
    inferred_type = 4294967296i64;

    // ミュータブルな変数は変更できる
    let mut mutable = 12; // ミュータブルなi32
    mutable = 21;
    // mutable = true; // 型は変更できない

    // 変数はシャドーイングで上書きできる
    let mutable = true;
}

fn literals_and_operators() {
    // 整数の加算
    println!("1 + 2 = {}", 1u32 + 2);
    // 1 + 2 = 3

    // 整数の減算
    println!("1 - 2 = {}", 1i32 - 2);
    // 1 - 2 = -1

    // 科学表記法
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    // 1e4 is 10000, -2.5e-3 is -0.0025

    // ブール値のロジック
    println!("true AND false is {}", true && false); // true AND false is false
    println!("true OR false is {}", true || false); // true OR false is true
    println!("NOT true is {}", !true); // NOT true is false

    // ビット演算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0011 AND 0101 is 0001
    println!("0011 OR  0101 is {:04b}", 0b0011u32 | 0b0101); // 0011 OR  0101 is 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0011 XOR 0101 is 0110
    println!("1 << 5 is {}", 1u32 << 5); // 1 << 5 is 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x80 >> 2 is 0x20

    // 読みやすくするためにアンダースコアを利用
    println!("One million is written as {}", 1_000_000u32); // One million is written as 1000000
}

// タプルは異なる型の値のコレクション。
fn tuples() {
    // タプルは異なる型を指定できる
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 'a', true);

    // タプルのインデックスで値を取得できる
    println!("Long tuple first value: {}", long_tuple.0); // Long tuple first value: 1
    println!("Long tuple second value: {}", long_tuple.1); // Long tuple second value: 2

    // タプルはタプルを要素に持つことができる
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // tuple of tuples: ((1, 2, 3), (4, -1), -2)

    // 要素が13以上の長いタプルはプリントできない（コンパイルエラーになる）
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    // タプルを分解してバインディングをできる
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    // 1, "hello", 4.5, true

    use std::fmt;

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }

    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    // Matrix(1.1, 1.2, 2.1, 2.2)
    println!("Matrix:\n{}", matrix);
    // Matrix:
    // ( 1.1 1.2 )
    // ( 2.1 2.2 )
    println!("Transpose:\n{}", transpose(matrix));
    // Transpose:
    // ( 1.1 2.1 )
    // ( 1.2 2.2 )
}

// 配列は同じ型の値のコレクション。配列の長さはコンパイル時に決まる
// スライスは配列に似ている。ポインタとスライスの長さで表す。スライスの長さはコンパイル時に決まらない
fn array_and_slices() {
    // 固定サイズの配列（型指定は任意）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // すべての要素を同じ値で初期化する（0の値で500の長さの配列を作成）
    let ys: [i32; 500] = [0; 500];

    // インデックスは0から始まる
    println!("First element of the array: {}", xs[0]);
    // First element of the array: 1
    println!("Second element of the array: {}", xs[1]);
    // Second element of the array: 2

    // len() は配列の要素数を返す
    println!("Number of elements in array: {}", xs.len());
    // Number of elements in array: 5

    // 配列はスタックに割り当てられる
    use std::mem;
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    // Array occupies 20 bytes


    // 配列は自動的にスライスとして借用される
    // スライスを借用する関数
    fn analyze_slice(slice: &[i32]) {
        println!("First element of the slice: {}", slice[0]);
        println!("The slice has {} elements", slice.len());
    }

    analyze_slice(&xs);
    // First element of the slice: 1
    // The slice has 5 elements

    // スライスは配列の範囲を指定できる
    analyze_slice(&ys[1 .. 4]);
    // First element of the slice: 0
    // The slice has 3 elements

    // 空のスライス &[]
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // 要素外のアクセスはランタイムエラーを発生させる
    // println!("{}", xs[5]);

    // 配列は get() メソッドにより安全に値を取得できる
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
    // 0: 1
    // 1: 2
    // 2: 3
    // 3: 4
    // 4: 5
    // Slow down! 5 is too far!
}
