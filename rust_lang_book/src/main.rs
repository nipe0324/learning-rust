use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 型TがDisplayトレイトとPartialOrdトレイトを実装しているときのみ、
// cmp_displayメソッドを実装する
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p1 = Pair::new(1, 2);
    p1.cmp_display();

    let p2 = Pair::new(vec![1], vec![2]);
    // p2.cmp_display(); は呼び出せない
}
