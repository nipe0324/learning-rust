# 構造体

- 構造体は、意味のあるグループを形成する複数の関連した値をまとめ、名前付けできる独自のデータ型です。
- `struct`キーワードで構造体を定義する。
- 構造体を使用することで、関連のあるデータを相互に結合させたまま、各部品に名前を付け、コードを明確にすることができる。
- また、メソッドにより、構造体のインスタンスが行う動作を指定したり、関連関数により構造体に関連する関数の名前空間わけすることができる。

```rs
// ユーザアカウントに関する情報を保持する構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
- 構造体のインスタンス生成は、構造体名とブロック(`{}`)で生成できる。
- フィールド名と変数数名が同じ場合はフィールド省略記法が使える

```rs
// フィールド名と変数数名が同じ場合はフィールド省略記法が使える
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
```

- 構造体の更新記法を利用で他のインスタンスから新しいインスタンスを生成できる

```rs
let user2 = User {
    username: String::from("another123"),
    email: String::from("another@example.com"),
    ..user1 // 更新帯の更新記法
};
```

- フィールドに紐付けられた名前がなく、フィールドの型だけの構造体をタプル構造体という

```rs
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
let white = Color(255, 255, 255);
```

- 構造体の内容のデバッグ出力

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
    // rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1);
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}
```

## メソッド

- 構造体にはメソッドを定義できる
- `impl`ブロックで内に`fn`でメソッドを定義できる
- メソッドの場合、第一引数は`self`で呼び出し元の構造体自体を表す

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- Rustのメソッド呼び出しでは、自動参照および参照外しが自動で行われる
- `object.something()`と呼び出すと、コンパイラはメソッドのシグニチャに合うように自動的に、`&`か`&mut`か`*`を付与する
- 受け手とメソッド名が与えられれば、コンパイラは確実にメソッドが「読み込み専用(`&self`)」か、「書き込みもする(`&mut self`)の」か、 「所有権を奪う(&self)のか」判断できるわけです。

```rs
// 前者はコンパイラが自動で解決してくれるのでどっちも同じになる
p1.distance(&p2);
(&p1).distance(&p2);
```

## 関連関数

- `impl`ブロック内に`self`を引数にとらない関数を定義できる。構造体に関連付けられているのでこれを関連関数と呼ばれる
- 関連関数は、構造体の新規インスタンスを返すコンストラクタによく使用される

```rs
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let rect = Rectangle::square(40);
```
