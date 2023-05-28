use crate::println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// Interrupt Descriptor Table(IDT)を初期化する
pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
}

// ブレークポイント例外のハンドラー
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame)
}
