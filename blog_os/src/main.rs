#![no_std] // OSの機能に依存している標準ライブラリにリンクしない
#![no_main] // 通常のエントリポイントを使わないようにする
#![feature(custom_test_frameworks)] // カスタムテストフレームワークを使うための機能
#![test_runner(blog_os::test_runner)] // カスタムテストランナーを使う
#![reexport_test_harness_main = "test_main"] // テストランナーのエントリポイントを`test_main`にする

use blog_os::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address(),
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}

// パニック時に呼ばれる関数
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

// テストのパニック時に呼ばれる関数
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
    // assert_eq!(0, 1);
}
