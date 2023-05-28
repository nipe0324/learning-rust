use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    // タスクステートセグメント（TSS）というレガシーな構造体を作成する
    // TSSは割り込みスタックテーブル（IST）を指すポインタを持っている
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // メモリ管理の仕組みがないので、新しいスタックを確保する適切な方法がない
            // 今回は、スタックのストレージとして、 static mut な配列を使っている
            // 注意点としてガードページを持たないので、スタックオーバーフローがスタックより
            // 下のメモリを破壊するかもしれない
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    // グローバルディスクリプタテーブル（GDT）を作成する
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::tss_segment(&TSS));
        gdt
    };
}

pub fn init() {
    GDT.load();
}
