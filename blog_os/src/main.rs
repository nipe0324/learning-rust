#![no_std]  // OSの機能に依存している標準ライブラリにリンクしない
#![no_main] // 通常のエントリポイントを使わないようにする

use core::panic::PanicInfo;

// リンカーはデフォルトで、`_start`という名前の関数を探すので
// この関数がエントリポイントになる
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// パニック時に呼ばれる関数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
