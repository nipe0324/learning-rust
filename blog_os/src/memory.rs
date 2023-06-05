use x86_64::{
    structures::paging::{OffsetPageTable, PageTable},
    PhysAddr, VirtAddr,
};

/// 新しいOffsetPageTableを初期化する
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// 有効なレベル4テーブルへの可変参照を返す
unsafe fn active_level_4_table(phsical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = phsical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

// /// 仮想アドレスを対応する物理アドレスに変換する。
// /// アドレスがマップされていないなら`None`を返す
// pub unsafe fn translate_addr(addr: VirtAddr, phsical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     translate_addr_inner(addr, phsical_memory_offset)
// }

// fn translate_addr_inner(addr: VirtAddr, phsical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     use x86_64::registers::control::Cr3;
//     use x86_64::structures::paging::page_table::FrameError;

//     let (level_4_table_frame, _) = Cr3::read();

//     let table_indexes = [
//         addr.p4_index(),
//         addr.p3_index(),
//         addr.p2_index(),
//         addr.p1_index(),
//     ];

//     let mut frame = level_4_table_frame;

//     // 複数層のページテーブルをたどる
//     for &index in &table_indexes {
//         // フレームをページテーブルの参照に変換する
//         let virt = phsical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe { &*table_ptr };

//         // ページテーブルエントリを読んで、`frame`を更新する
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         };
//     }

//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }
