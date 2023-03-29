fn main() {
    let x = vec![1, 2, 3];

    // クロージャのequal_to_xはx変数を利用できる
    let equal_to_x = |z| z == x;

    // move を利用すると、所有権を移動させることがｄけいる
    // let equal_to_x = move |z| z == x;

    let y =  vec![1, 2, 3];

    assert!(equal_to_x(y));
}
