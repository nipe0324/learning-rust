use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

/// ブートローダのメモリマップから、使用可能なフレームを返すFrameAllocator
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// 渡されたメモリマップからFrameAllocatorを作成する
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// メモリマップによって指定されたusableなフレームのイテレータを返す
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // メモリマップからusableな領域を取得する
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        // それぞれの領域をアドレス範囲に変換する
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        // フレームの開始アドレスのイテレータへと変換する
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        // 開始アドレスから`PhysFrame`型を作る
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

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

/// 常に`None`を返すFrameAllocator
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

/// 与えられたページをフレーム`0xb8000`に試しにマップする
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: unsafeであり、テストのためにのみ行う
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
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
