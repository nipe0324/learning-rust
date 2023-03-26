#[derive(Debug)]
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("drop MyStruct: {:?}", self);
        // self.drop(); // 書き方あっているかわからん
    }
}

// 所有権が移動する
fn do_sometihng(s: MyStruct) {
    println!("called do_something: {:?}", s);
} // s1のdropがここで呼ばれる

// 参照で渡すので所有権が移動しない
fn borrow_sometihng(s: &MyStruct) {
    println!("called borrow_sometihng: {:?}", s);
}

fn main() {
    println!("1. move ownership");
    let s1 = MyStruct {};
    do_sometihng(s1);
    println!("after do_something\n");

    println!("2. reference ownership");
    let s2 = MyStruct {};
    borrow_sometihng(&s2);
    println!("after borrow_sometihng");
} // s2のdropがここで呼ばれる
