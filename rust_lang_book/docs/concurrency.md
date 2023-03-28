# 並行性

- Rustの主な目標の１つに、並行性を安全かつ効率的に扱うことがある。
- 並行プログラミングは、多くのコンピュータが複数のプロセッサの利点を活かすようになれ、重要度を増している
- Rustチームは、所有権と型システムは、メモリ安全性と並行性問題を管理に役立つ強力なツールであることを発見した
- 具体的には、所有権と型チェックを活用することで、多くの並行性エラーは、実行時エラーではなくコンパイラ時にエラーにできるようになった。

## スレッド

- 多くの現代のOSでは、実行中のプログラムのコードはプロセスで走り、OSは同時に複数のプロセスを管理する
- 自分のプログラム内で、独立した部分を同時に実行できるように、スレッドで走らせる。
- Rustの標準ライブラリは、1:1スレッドの実装のみを提供している。Rustではほぼゼロのランタイムが必要とされるのでわりきっている。

```rs
use std::thread;
use std::time::Duration;

fn main() {
    // spawnで新規スレッドを生成する
    // メインスレッドが終了すると、新規スレッドは途中でも停止する
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the main thread!
// hi number 2 from the spawned thread!
// hi number 3 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the main thread!
// hi number 4 from the spawned thread!
```

- `join`ハンドルで全スレッドの終了を待つことができる
- `join`を呼び出すと、ハンドルが表すスレッドが終了するまで現在実行中のスレッドをブロックする

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    println!("finish man thread");
}

// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the main thread!
// hi number 2 from the spawned thread!
// hi number 3 from the spawned thread!
// hi number 3 from the main thread!
// hi number 4 from the spawned thread!
// hi number 4 from the main thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
// finish man thread
```

- あるスレッドのデータを別のスレッド使用できるようになるので、`move`クロージャは`thread::spawn`とともによく使用される。

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // moveを使うことで、クロージャに使用されている値vの所有権を強制的に奪わせている
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    drop(v);

    handle.join().unwrap();
}
// Here's a vector: [1, 2, 3]
```

## チャンネル

- 安全な並行性を保証する１つのアプローチとしてメッセージ受け渡しがある。
- メッセージ受け渡しの並行性を達成するためにチャンネルが用意されている

```rs
use std::thread;
use std::sync::mpsc;

fn main() {
    // チャンネルの作成
    // mpseはmultiple producer, single consumerの頭文字
    // txは転送機(送信側)、rxは受信機(受信側)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // メッセージの送信
        tx.send(val).unwrap();
    });

    // メッセージの受け取り、recv()はメッセージの受信までブロックする
    // try_recv()はブロックされない
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    // Got: hi
}
```

- 複数の転送機から送る。送信機を`mpsc::Sender::clone()`でクローンする

```rs
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
// Got: hi
// Got: more
// Got: from
// Got: messages
// Got: the
// Got: for
// Got: thread
// Got: you
```


## ミューテックス

- ミューテックス(mutex)は、どんなときも１つのスレッドにしかデータへのアクセスを許可しないという"mutual exclusion"（相互排他）の省略形。
- ミューテックスを正しく使うにはテクニックが必要だが、Rustの型システムと所有権規則のおかげで、ロックとアンロックをおかしくすることはない

```rs
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Arcはスレッドセーフなスマートポインタ
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // ロックをとる
            let mut num = counter.lock().unwrap();

            *num += 1;
        }); // 自動的にロックが解放される

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// Result: 10
```
