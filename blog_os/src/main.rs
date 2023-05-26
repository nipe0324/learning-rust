#![no_std] // OSの機能に依存している標準ライブラリにリンクしない
#![no_main] // 通常のエントリポイントを使わないようにする
#![feature(custom_test_frameworks)] // カスタムテストフレームワークを使うための機能
#![test_runner(crate::test_runner)] // カスタムテストランナーを使う
#![reexport_test_harness_main = "test_main"] // テストランナーのエントリポイントを`test_main`にする

use core::panic::PanicInfo;

mod vga_buffer;

// リンカーはデフォルトで、`_start`という名前の関数を探すので
// この関数がエントリポイントになる
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// パニック時に呼ばれる関数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
