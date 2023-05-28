use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

// Interrupt Descriptor Table(IDT)を初期化する
pub fn init_idt() {
    IDT.load();
}

// ブレークポイント例外のハンドラー
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame)
}

#[test_case]
fn test_breakipoint_exception() {
    // ブレークポイント例外を発生させる
    x86_64::instructions::interrupts::int3();
}
