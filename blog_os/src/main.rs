#![no_std] // OSの機能に依存している標準ライブラリにリンクしない
#![no_main] // 通常のエントリポイントを使わないようにする

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello, World!";

// リンカーはデフォルトで、`_start`という名前の関数を探すので
// この関数がエントリポイントになる
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// パニック時に呼ばれる関数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga_buffer::print_something();

    loop {}
}
