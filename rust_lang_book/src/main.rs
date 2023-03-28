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
